import * as wasm from 'pow';

const API_GET_POW = '/api/pow';
const API_SIGN_IN = '/api/signin';
const API_SIGN_UP = '/api/signup';

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

const genPoW = async () => {
  return fetch(API_GET_POW)
    .then(resp => resp.json())
    .then(data => wasm.gen_pow(data.difficulty, data.phrase))
    .then(pow => JSON.parse(pow));
};

const signup = async () => {
  let username = document.getElementById('username').value;
  let password = document.getElementById('password').value;
  let rePassword = document.getElementById('re-password').value;
  let email = document.getElementById('email').value;
  isBlankString(email, 'email');
  isBlankString(rePassword, 'password');
  isBlankString(username, 'username');
  isBlankString(password, 'password');

  if (password !== rePassword) {
    alert("entered passwords don't match");
  } else {
    let pow = await genPoW();
    console.log(`from signup: PoW: ${pow}`);
    const payload = {
      username: username,
      password: password,
      email_id: email,
      pow: pow,
    };

    sendSignUp(payload);
  }
};

const sendSignUp = async payload => {
  fetch(API_SIGN_UP, genJsonPayload(payload)).then(resp => {
    if (resp.ok) {
      alert('signed up');
    } else {
      resp.json().then(resp => alert(`Error: ${resp.error}`));
    }
  });
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
      resp.json().then(resp => alert(`Error: ${resp.error}`));
    }
  });
};

window.signin = signin;
window.signup = signup;
