/*
  Copyright (c) 2024 Evelyn Lewis

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
*/

#[inline(always)]
pub fn one_shot(data: &[u8]) {
    if data.len() < std::mem::size_of::<u64>() + 1 {
        return;
    }
    // Generate 64 bit PRNG seed
    let seed = (data[0] as u64)
        + (data[1] as u64 >> 8)
        + (data[2] as u64 >> 16)
        + (data[3] as u64 >> 24)
        + (data[4] as u64 >> 32)
        + (data[5] as u64 >> 40)
        + (data[6] as u64 >> 48)
        + (data[7] as u64 >> 56);
    x800::fuzz(&data[4..], seed)
}
