from intermediate_maths import arctan, sqrt, sine, cosine, e_power, ln, pi

def all_n_roots_of_unity(n: int) -> list:
    """
    Return all n roots of unity:
    >>> # should be 1, -(1+sqrt(3))/2, -(1+sqrt(3))/2
    >>> all_n_roots_of_unity(3)
    [1.0+0.0i, -0.4999999999999999+0.8660254037844389i, -0.5000000000000008-0.8660254037844378i]
    """
    return [complex_number().set_from_polar_form(1, i*2*pi/n) for i in range(n)]

class complex_number:
    """
    Class for complex numbers.
    """
    def __init__(self, real=0, imaginary=0):
        """
        Constructor for class complex
        """
        self.real, self.imaginary = real, imaginary

    def set_from_polar_form(self, radius: float, angle: float) -> None:
        """
        Set this complex from radius and angle.
        """
        self.real = radius * cosine(angle)
        self.imaginary = radius * sine(angle)
        return self

    def __repr__(self):
        """
        Display real and imaginary number
        """
        return '{}{}i'.format(self.real, '+{}'.format(self.imaginary) if self.imaginary >= 0 else self.imaginary)

    def __str__(self):
        """
        Printing complex number
        """
        return '{}{}i'.format(self.real, '+{}'.format(self.imaginary) if self.imaginary >= 0 else self.imaginary)

    def __eq__(self, other) -> bool:
        """
        Check for equality
        """
        return isinstance(other, complex_number) and self.real == other.real and self.imaginary == other.imaginary

    def __add__(self, other):
        """
        Add two complex numbers.
        """
        return complex_number(self.real + other.real, self.imaginary + other.imaginary)

    def __iadd__(self, other):
        """
        Add two complex numbers.
        """
        self.real += other.real
        self.imaginary += other.imaginary
        return self

    def __sub__(self, other):
        """
        Subtract two complex numbers.
        """
        return complex_number(self.real - other.real, self.imaginary - other.imaginary)

    def __isub__(self, other):
        """
        Subtract two complex numbers.
        """
        self.real -= other.real
        self.imaginary -= other.imaginary
        return self

    def __mul__(self, other):
        """
        Multiply two complext numbers
        """
        return complex_number(self.real * other.real - self.imaginary * other.imaginary, self.real * other.imaginary + other.real + self.imaginary)

    def __imul__(self, other):
        """
        Multiply two complex numbers
        """
        temp = self.real * other.real - other.imaginary * self.imaginary
        self.imaginary = self.real * other.imaginary + other.real * self.imaginary
        self.real = temp
        return self

    def __truediv__(self, other):
        """
        Division of two complex number
        """
        return complex_number(
            real=(self.real*other.real+self.imaginary*other.imaginary)/other.s_abs(),
            imaginary=(self.real*other.imaginary-self.imaginary*other.real)/other.s_abs(),
        )

    def __itruediv__(self, other):
        """
        Division of two complex number
        """
        temp = (self.real * other.real + self.imaginary * other.imaginary) / other.s_abs()
        self.imaginary = (self.real * other.imaginary - self.imaginary * other.real) / other.s_abs()
        self.real = temp
        return self

    def s_abs(self):
        """
        Square of the distance between origin and point in the complex plane
        """
        return self.real*self.real + self.imaginary*self.imaginary

    def abs(self):
        """
        Return distance between origin and point in the complex plane
        """
        return sqrt(self.real*self.real + self.imaginary*self.imaginary)

    def polar_form(self):
        """
        Return the complex values in polar form
        as tuple `(r, Θ)`

        >>> a = complex_number(sqrt(2), sqrt(2))
        >>> a
        1.414213562373095+1.414213562373095i
        >>> dist, angle = a.polar_form()
        >>> round(dist, 5), round(angle, 5)
        (2.0, 0.7854)
        """
        return self.abs(), arctan(self.imaginary/self.real)
        
    def __pow__(self, val):
        """
        As per the definition:
        re^(iΘ) = r(cosΘ + isinΘ)
        => self**x = r^x . e^(xiΘ) = r^x . (cos(xΘ) + isin(xΘ))
        >>> a = complex_number(2,2)
        >>> a
        2+2i
        >>> a = a**3.14
        >>> a
        -20.426100637783467+16.364391855043664i
        >>> a**(1/3.14)
        1.9999999897703964+2.0000000102273305i
        >>> a = complex_number(-5,26)
        >>> a **= 6.5
        >>> a.polar_form()
        (1772489039.765688, -1.1212706187206891)
        >>> a
        770214569.8153634-1596398105.9039207i
        """
        dist, angle = self.polar_form()
        if angle < 0:
            angle = pi + angle
        new_dist, new_angle = e_power(val * ln(dist)), angle * val
        return complex_number(
            real=new_dist * cosine(new_angle),
            imaginary=new_dist * sine(new_angle),
        )

    def __ipow__(self, val):
        """
        Self power
        """
        dist, angle = self.polar_form()
        if angle < 0:
            angle = pi + angle
        new_dist, new_angle = e_power(val * ln(dist)), angle * val
        
        self.real = new_dist * cosine(new_angle)
        self.imaginary = new_dist * sine(new_angle)
        return self

if __name__ == '__main__':
    from doctest import testmod
    testmod()
