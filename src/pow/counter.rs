/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix::prelude::*;
use lazy_static::*;
use std::time::Duration;

use super::Levels;

const POW_SESSION_DURATION: u64 = 20;

lazy_static! {
    pub static ref DURATION: Duration = Duration::new(POW_SESSION_DURATION, 0);
}

#[derive(Message)]
#[rtype(result = "u32")]
pub struct Visitor;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DeleteVisitor;

pub struct Counter {
    visitor_count: usize,
    levels: Levels,
}

impl Default for Counter {
    fn default() -> Self {
        Counter {
            visitor_count: 0,
            levels: Levels::default(),
        }
    }
}

impl Actor for Counter {
    type Context = Context<Self>;

    //    fn started(&mut self, ctx: &mut Self::Context) {
    //        ctx.set_mailbox_capacity(usize::MAX / 2);
    //    }
}

impl Handler<Visitor> for Counter {
    type Result = u32;
    fn handle(&mut self, _: Visitor, ctx: &mut Self::Context) -> Self::Result {
        use actix::clock::delay_for;

        self.visitor_count += 1;

        let addr = ctx.address();

        let wait_for = async move {
            delay_for(*DURATION).await;
            addr.send(DeleteVisitor).await.unwrap();
        }
        .into_actor(self);
        ctx.spawn(wait_for);

        if self.visitor_count > self.levels.threshold() {
            self.levels.focus();
        } else {
            self.levels.relax();
        }

        self.levels.get_difficulty()
    }
}

impl Handler<DeleteVisitor> for Counter {
    type Result = ();
    fn handle(&mut self, _msg: DeleteVisitor, _ctx: &mut Self::Context) -> Self::Result {
        if self.visitor_count > 0 {
            self.visitor_count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn race(addr: Addr<Counter>, count: Levels) {
        for _ in 0..count as usize - 1 {
            let _ = addr.send(Visitor).await.unwrap();
        }
    }
    #[actix_rt::test]
    async fn counter_focus_works() {
        let four = Levels::Four.get_difficulty();
        let three = Levels::Three.get_difficulty();
        let two = Levels::Two.get_difficulty();
        let one = Levels::One.get_difficulty();

        let addr = Counter::default().start();

        let mut difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, one);

        let addr = Counter::default().start();
        race(addr.clone(), Levels::One).await;
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, one);

        let addr = Counter::default().start();
        race(addr.clone(), Levels::Two).await;
        addr.send(Visitor).await.unwrap();
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, two);

        let addr = Counter::default().start();
        race(addr.clone(), Levels::Three).await;
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, three);

        let addr = Counter::default().start();
        race(addr.clone(), Levels::Four).await;
        addr.send(Visitor).await.unwrap();
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, four);
    }

    #[actix_rt::test]
    async fn counter_relax_works() {
        use actix::clock::delay_for;
        let four = Levels::Four.get_difficulty();
        let three = Levels::Three.get_difficulty();
        let two = Levels::Two.get_difficulty();
        let one = Levels::One.get_difficulty();

        let addr = Counter::default().start();

        let mut difficulty_factor = addr.send(Visitor).await.unwrap();

        let addr = Counter::default().start();
        race(addr.clone(), Levels::Four).await;
        addr.send(Visitor).await.unwrap();
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, four);

        delay_for(*DURATION).await;

        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, three);
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, two);
        difficulty_factor = addr.send(Visitor).await.unwrap();
        assert_eq!(difficulty_factor, one);
    }
}
