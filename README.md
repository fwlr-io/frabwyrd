# Frabwyrds

> _’Twas brillig, and the slithy toves_
>
>      _Did gyre and gimble in the wabe:_
>
> _All mimsy were the borogoves,_
>
>      _And the mome raths outgrabe._
>

### Generating lots of interesting nonsense words

This [presentation](demo/presentation.pdf) I gave is a good introduction to the project.

Check out the [demo word list](demo/demo_word_list.txt) for more examples.

[The OG Karpathy blogpost](http://karpathy.github.io/2015/05/21/rnn-effectiveness/) ([code](https://github.com/karpathy/char-rnn)) that started me on this project. It takes some code archeology to get decade-old projects running.

There is a more-recently-updated [optimised version](https://github.com/jcjohnson/torch-rnn). I had success running it with [nvidia-docker](https://github.com/NVIDIA/nvidia-docker) on a p2.xlarge AWS instance using "Deep Learning Base AMI (Ubuntu 18.04) Version 52.0 (ami-0d607ac86033c7b37)" for the image.


### Texts used
* The Collected Works of H.P. Lovecraft (~4MB, ~712k words)
* The Collected Works of Sir Arthur Conan Doyle (~3MB, ~578k words)
* The Collected Works of Edgar Allan Poe (~1MB, ~190k words)
* The King In Yellow by Robert W. Chambers (~400KB, ~71k words)

Texts from the fantastic [WikiSource website](https://wikisource.org/wiki/Main_Page).

Training texts were pre-processed by converting characters to ASCII, or removing them if they had no close equivalent.

### Dictionaries used
* Medium size, no mis-spellings (105,056 words)
* Default size, common mis-spellings (128,628 words)
* Huge size, common and rare mis-spellings (358,842 words)
* Insane size, all mis-spellings (677,131 words)

[Spell-Check Oriented Word Lists](http://wordlist.aspell.net/scowl-readme/), from the fantastic Kevin Atkinson.

The dictonaries were used to filter out all real words, leaving behind just the nonsense almost-words.
