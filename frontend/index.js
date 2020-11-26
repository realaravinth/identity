import * as wasm from 'pow';

const API_GET_POW = '/api/pow';

console.log('executing');
fetch(API_GET_POW)
  .then(data => data.json())
  .then(data => {
    console.log(data);
    let pow = wasm.gen_pow(
      data.difficulty.toLocaleString('fullwide', {useGrouping: false}),
      data.phrase,
    );
    console.log(pow);
  });
