# In your JavaScript code, you can then import the Rust function and use it to perform operations on TensorFlow.js tensors. Here's an example of how you might use the sum function in JavaScript:

import { sum } from './path/to/your/rust_module.js';

const tensor = tf.tensor([1, 2, 3, 4, 5]);
const sum = sum(tensor.dataSync());
console.log(sum); // 15
