---
author: Dorian Beauchesne
date: 31/07/2023
title: Low Language Model
language: en
---


# Low Language Model (LLM)
> The technology behing ChatGPT, Google Bard, Meta Llama and far more.

**Large Language model** is the technology of artificial intelligence often used to work with massive amount of text data. It uses deep neural netwoks. 


Acording to wikipedia[^1], they work by "taking an input text and repeatedly predicting the next token or word"
**LLM** are mainly used for : 
- Text generation
- Translation
- Summarization

# Transformer architecture
Developped by Google in 2017, and announced in their paper *Attention Is All You Need[^2]*...  
Basically, **transformers** are encoder/decoder blocks that process data and allows the LLM to find correlation between inputs.
![The Transformer Model diagram](https://machinelearningmastery.com/wp-content/uploads/2021/08/attention_research_1.png)  
*The Transformer architecture proposed by Google*

### Encoding
The classic encoding algorithm is **bytes-pair** encoding. The output of the encoding block is a "embedded vector", which is a dense and continious representation of each word.

### Attention
They are using a concept calaled **self-attention** to find relationship between different tokens. The attention mechanism allows the network to focus on certain sequences of the input.

It uses 3 ponderation matrices to ponderate the inputs : **Q**uery, **K**ey & **V**alue.  
The **Q**uery matrix represents the importance of each word , the **K**ey matrix represents the influence of each word on the others and the output of the attention mechanism is obtained by calculating **V = f(Q * K)**.

Multiples attention heads are used in parrallel in order to capture different relation types. Each of them has its own matrices.


# Datasets & parameters
The datasets used to trained this models are huge (some petabytes) and since they are not very clever, they need bilions of parameters to "understand" how languages kind of works. For instance, the number of parameters used in recent models are : 
- GPT-4: 1 trillion
- GPT-3: 175 billion
- GPT-2: 2 billion
- BERT: 345 million

[^1]: https://en.wikipedia.org/wiki/Large_language_model  
[^2]: https://arxiv.org/pdf/1706.03762.pdf