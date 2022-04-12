Frabwyrds

RNN
article http://karpathy.github.io/2015/05/21/rnn-effectiveness/
original github https://github.com/karpathy/char-rnn
the optimised version https://github.com/jcjohnson/torch-rnn
run on nvidia-docker https://github.com/NVIDIA/nvidia-docker


amazon AWS
p2.xlarge 12G 4x 61GiB

Ubuntu 18.04 GPU instance
Deep Learning Base AMI (Ubuntu 18.04) Version 52.0
ami-0d607ac86033c7b37


next steps
how to split string into words
make a clean up script for processing text (squash multiple newlines down to 2, strip "newlines between non-period characters" for lovecraft specifically, strip non-ASCII chars, replace single quotation marks with ' and double quotation marks with ", and maybe pare down to just lowercase letters and apostrophes)


### Texts used
* The Collected Works of H.P. Lovecraft (~4MB, ~712k words)
* The Collected Works of Sir Arthur Conan Doyle (~3MB, ~578k words)
* The Collected Works of Edgar Allan Poe (~1MB, ~190k words)
* The King In Yellow by Robert W. Chambers (~400KB, ~71k words)

Texts from the fantastic [WikiSource website](https://wikisource.org/wiki/Main_Page)

### Dictionaries used
* Medium size, no mis-spellings (105,056 words)
* Default size, common mis-spellings (128,628 words)
* Huge size, common and rare mis-spellings (358,842 words)
* Insane size, all mis-spellings (677,131 words)

[Spell-Check Oriented Word Lists](http://wordlist.aspell.net/scowl-readme/), from the fantastic Kevin Atkinson