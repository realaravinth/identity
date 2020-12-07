import * as wasm from 'pow';

const API_GET_POW = '/api/pow';
const API_SIGN_IN = '/api/signin';
const API_SIGN_UP = '/api/signup';

let pow = null;

const isBlankString = (value, field) => {
  if (!value.replace(/\s/g, '').length) {
    alert(`${field} can't be empty`);
  }
};

const genJsonPayload = payload => {
  let value = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  };
  return value;
};

console.log('executing');
const genPoW = async () => {
  if (pow === null) {
    let response = await fetch(API_GET_POW);
    let data = await response.json();
    console.log(data);
    pow = wasm.gen_pow(dletata.difficulty, data.phrase);
  }
  console.log(pow);
};

const signin = () => {
  let username = document.getElementById('username').value;
  let password = document.getElementById('password').value;
  isBlankString(username, 'username');
  isBlankString(password, 'password');

  const payload = {
    username: username,
    password: password,
  };

  console.log('Sending payload');
  sendSignIn(payload);
};

const sendSignIn = async payload => {
  fetch(API_SIGN_IN, genJsonPayload(payload)).then(resp => {
    if (resp.ok) {
      alert('signed in');
    } else {
      alert(`Error &{resp.status}`);
    }
  });
};

genPoW().then(() => console.log('s'));

window.signin = signin;

module.exports = {
  siginin: signin,
};
