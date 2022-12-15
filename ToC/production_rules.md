A production rules is a rules that can be applied to generate certain kind of string sequences. [A more concise definition can be seen here](https://en.wikipedia.org/wiki/Production_(computer_science)).

Here, we define certain valid inputs of terminal symbols and non-terminal symbols to generate a kind of string.

# Defining grammar rules.
Assume that start symbol is $S$, with set of terminal symbols (with small letters, for e.g., $\{a,b\}$) and we need to generate a string that ends with $ab$, then we can create a generator as:
$$
\begin{matrix}
S\rightarrow aS\ |\ bS\\S\rightarrow ab
\end{matrix}
$$
This ensures that whatever the values are being substituted, these values will terminate with the sequence $ab$.

# Applications
Creating a generator for JSON. Each of these non-terminating symbols will acts as a function that will parse the JSON string and construct them.

- Non-terminating symbols will be denoted with bar (e.g., $\bar A, \bar B$.)
- We'll be detecting numbers ($\bar N$), boolean values ($\bar B$), strings ($\bar S$), objects ($\bar O$) and arrays ($\bar A$). With that, we'll also denote the empty sequence with symbol $\phi$.
- Considering we receive the JSON data stringified.

## Numbers
Terminal symbols: $\{0,1,2,3,4,5,6,7,8,9,e,E,.,+,-\}$ are set of used characters in number. The floating values are denoted by $e$, (.e.g., 1e9, or 1E9 is basically $10^9$).

Also let us assume ${\Sigma_N}$ as numbers ($0|1|2|3...|9$), and 
We'll also denote sign $S\rightarrow +, S\rightarrow -, S\rightarrow \phi$

First we'll define positive integer ($\bar{I}$) as:
$$
\begin{matrix}
\bar{I}\rightarrow1\bar I\ |\ 2\bar I\ |\ ...\ |\ 9\bar{I} \implies \bar{I}\rightarrow (\Sigma_N\ - \{0\})\bar I\\
\bar I\rightarrow\Sigma_N&, \text{terminating with either any number}
\end{matrix}
$$
With this, we denote exponent part as:
$$
\bar E\rightarrow Se\bar I\ |\ SE\bar I
$$
With this, numbers can be detected as:
$$
\begin{array}{cl}
\bar N\rightarrow\ S \bar I&\text{, integer, 2345}\\
\bar N\rightarrow\ S.\bar I\ |\ S0.\bar I &\text{, decimal values < 1, e.g., 0.124}\\
\bar N\rightarrow\ S\bar I.\bar I &\text{, decimal values}\geq\text{0, e.g., 0.124, 25.2345}\\
\bar N\rightarrow\ S\bar IS\bar E\ &\text{, decimal values with exponent: e.g., 321e1, -2e-3}\\
\bar N\rightarrow\ S.\bar IS\bar E |\ S0.\bar IS\bar E &\text{, decimal values, e.g., 0.124E11, +.123e10,-0.777e1, etc}\\
\bar N\rightarrow\ S\bar I.\bar IS\bar E \\
\end{array}
$$
With this, we can see the pattern of how the numbers are generated. 
The non-terminal variable $\bar I$ is the only one that generates the values, while $\bar N$ uses multiple non-terminal variables to generate a valid number.

## Boolean and Null values
These are fairly simple values, only consisting of `true`, `false` and `null` values.
$$
\begin{array}{c}
\bar B\rightarrow\text{true}\\
\bar B\rightarrow\text{false}\\
\bar B\rightarrow\text{null}
\end{array}
$$
## String
Strings is a bit tricky because they are enclosed with a quotation marks. 
If there is a need of quotation in a string, then it has to be escaped with a back slash (\\) symbol.
Considering total symbols to be set of characters (say $\theta$), then, handling single quotation and double quotations differently, we'll consider double quotations as an example:
$$
\begin{array}{cl}
\bar G\rightarrow\theta\bar G\\
\bar S\rightarrow\text{''}\bar G\text{ ''},\\
\bar S\rightarrow\text{''}G\text{\\}\bar S
\end{array}
$$
Note that while storing these escaped character, we're not storing the backslashes.

## [[arrays_1d|Arrays]]
Arrays are defined as homogenous elements stored in consecutive locations. However, we're storing multiple types in one array.

Arrays, will be defined with non-terminating symbol $\bar A$ as:

$$
\begin{array}{c}
\bar {G_A}\rightarrow \bar S\\
\bar {G_A}\rightarrow \bar B\\
\bar {G_A}\rightarrow \bar A\\
\bar {G_A}\rightarrow \bar O\\
\bar {G_A}\rightarrow \bar N\\
\bar Q\rightarrow\phi\\
\bar Q\rightarrow\bar{G_A}Q\\
\bar A\rightarrow [Q]
\end{array}
$$
## Objects
$$
\begin{array}{}
\bar Q\rightarrow\bar S:\bar{G_A}\\
\bar Q\rightarrow\bar S:\bar{G_A},\bar Q\\
\bar O\rightarrow\{\bar Q\}\\
\bar O\rightarrow\{\}
\end{array}
$$
