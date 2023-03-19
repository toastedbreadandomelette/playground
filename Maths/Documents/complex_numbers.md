# Complex numbers (Complex set $\mathbb{C}$).
Complex numbers are defined by the specific numbers $z=a+i\cdot b$, where $a,b\in\mathbb{R}$, and the value $i$ is an imaginary unit defined as $i^2=-1$.

$a$ is defined as a real part $\Re({z})$  and $b$ is called an imaginary part $\Im({z})$.

In a plane, real value is denoted along $X$-axis and imaginary part along $Y$-Axis.

Technically all number types $\mathbb{N,W,Z,Q,F,I,R} \in \mathbb{C}$, where $b=0$.

## Polar form
An equivalent polar form of $a+i\cdot b$ is denoted as $r\cdot e^{i\theta}$.

Here $r=\sqrt{a^2+b^2}$ and $\theta=\arctan\left({\dfrac{b}{a}}\right)$.

This can be easily visualized in a plane, where $r$ is the vector from origin $(0,0)$, to $(a,b)$ and $\theta$ being angle between vector and $X$-Axis.

# Operations

Consider $z_1=a+i\cdot b,\quad z_2=c+i\cdot d$.

## Conjugate

A conjugate for $z=a+i\cdot b$ is $\bar{z}=a-i\cdot b$. It's a reflection of a vector along $X$-Axis.

## Addition & Substraction
$$
\begin{matrix}
z_1=a+i\cdot b,\quad z_2=c+i\cdot d\\
z_1+z_2=z=(a+c)+i\cdot (b+d)=u+i\cdot v\\
\implies u=a+c,\quad v=b+d
\end{matrix}
$$
Addition of vectors would look like a parallelogram.
$$
\begin{matrix}
z_1-z_2=(a-c)+i\cdot (b-d)\\
\implies u=a-c,\quad v=b-d
\end{matrix}
$$
## Multiplication
$$
z_1\cdot z_2 =(a+i\cdot b)\cdot (c+i\cdot d)=(a\cdot c-b\cdot d)+i \cdot(b\cdot c+a\cdot d)
$$
$\implies u=a\cdot c - b\cdot d$ and $v=b\cdot c-a\cdot d$.

## Division

$$
\begin{matrix}
\dfrac{z_1}{z_2}&=&\dfrac{a+i\cdot b}{c+i\cdot d}\\
&=&\dfrac{(a+i\cdot b)\cdot(c-i\cdot d)}{(c+i\cdot d)\cdot(c-i\cdot d)}&-\ (\text{multiply and divide by }\bar{z_2})\\
&=&\dfrac{(a\cdot c+b\cdot d)+i\cdot (b\cdot c-a\cdot d)}{c^2+d^2}
\end{matrix}
$$

$\implies u=\dfrac{a\cdot c+b\cdot d}{c^2+d^2}$ and $v=\dfrac{b\cdot c-a\cdot d}{c^2+d^2}$.

## Polar form

The polar form $r\cdot e^{i\theta}$ can be expanded to:

$$
\begin{matrix}
r\cdot e^{i\theta}&=&r\cdot\left(1+\dfrac{i\cdot\theta}{1!}-\dfrac{\theta^2}{2!}-\dfrac{i\cdot\theta^3}{3!}+\dfrac{\theta^4}{4!}+\cdots\right)\\
&=&r\cdot (\cos{\theta}+i\cdot\sin{\theta})
\end{matrix}
$$

If $z=r\cdot e^{i\theta}$, then $z^{n}=r^n\cdot e^{i\cdot n\theta}=r^n\cdot(\cos{(n\cdot\theta)}+i\cdot\sin{(n\cdot\theta)}),n\in\mathbb{R}$.

We express $e^{i\theta}$ as $\text{cis}(\theta)$ or in angle notation: $\angle\theta$  as well.

# Roots of Unity

The $n$ roots of unity $z^n-1=0$ are:

$$z^{1/n}=\text{cis}\left(\dfrac{2\pi p}{n}\right),\quad 0\leq p\lt n,\ r=1$$

These are also denoted by $\omega_n^p=\text{cis}\left(\dfrac{2\pi p}{n}\right)$. 

### Some results:

1. $\omega_{dn}^{dp}=\text{cis}\left(\dfrac{2\pi\cdot dp}{dn}\right)=\text{cis}\left(\dfrac{2\pi p}{n}\right)=\omega_n^p$.
2. $\forall\ n\in N,\ n\equiv0\pmod{2},\ \omega_n^{n/2}=\cos(\pi)+i\cdot \sin(\pi)=-1+i\cdot 0 =-1$ 
3. $\forall\ n\in N,\ n\equiv0\pmod{2}$
	- $(\omega_n^{p})^2=e^{2\pi p\cdot 2/n}=e^{\dfrac{2\pi p}{n/2}}=\omega_{n/2}^{p}$ 
	- $(\omega_n^{p+n/2})^2=\omega_n^{2p+n}=\omega_n^{2p}=(\omega_n^p)^2=\omega_{n/2}^p$
4. $\omega_n^{p+n/2}=\cos\left(\dfrac{2\pi\cdot\dfrac{n}{2}}{n}+\dfrac{2\pi p}{n}\right)+i\cdot \sin\left(\dfrac{2\pi\cdot\dfrac{n}{2}}{n}+\dfrac{2\pi p}{n}\right)=\cos\left(\pi+\dfrac{2\pi p}{n}\right)+i\cdot \sin\left(\pi+\dfrac{2\pi p}{n}\right)$
$=-\cos\left(\dfrac{2\pi p}{n}\right)-i\cdot \sin\left(\dfrac{2\pi p}{n}\right)=-\omega_{n}^{p}$ 
5. If $p\not\equiv 0\pmod n$, then:

$$
\begin{array}{cl}
\sum\limits_{i=0}^{n-1}(\omega_n^{p})^i&=&\dfrac{1-\omega_{n}^{np}}{1-\omega_{n}^p}\\
&=&0,\quad (\omega_n^{np}=1)
\end{array}
$$

```ad-note
The above results are apparent from some observations noted here:
- $\omega_n^{n+p}=\omega_n^p$
- $\omega_n^{2p}=e^{2\pi\cdot 2p/n}=(e^{2\pi/n})^2=(\omega_n^p)^2$
```

# $k^{th}$ Root of unity modulo $p$.


