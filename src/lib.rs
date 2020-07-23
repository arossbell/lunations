/*
Copyright (c) 2020 Adam Bell

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

/* CONSTANTS */

const BASE_DAY: f64 = 2449128.59;
const AVG_MONTH: f64 = 29.53058867;

/* LIBRARY FUNCTIONS */

// Do with what it returns as you want. It could be signed int-ish, but that could limit some uses, so do with it what you want ... because what's memory anyways?
pub fn jd_lunation(jd:f64) -> f64 {
    (jd - BASE_DAY) / AVG_MONTH
}

pub fn lunation_jd(lunation:f64) -> f64 {
    BASE_DAY + (AVG_MONTH * lunation)
}
