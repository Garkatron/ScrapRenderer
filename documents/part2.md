# Normals
Son una linea imaginaria perpendicular a el triangulo formado por los vertices.
Dadas dos lineas en un plano, el normal, es una tercera linea perpendicular a estas dos lineas.

**Cross Product**
```
???
```

Cada componente de nuestro vector normal, es un calculo de otros dos componentes (lineas).

```java
Nx = Ay * Bz - Az * By
Ny = Az * Bx - Ax * Bz
Nz = Ax * By - Ay * Bx
```

El normal debe ir en la direccion opuesta.

Cualquiera de los triangulos que tenga un normal que tenga una z negativa, podemos verla, pero si es positiva, no.
(0,0,1) No veo
(0,0,-1) SI veo

Necesitamos Nz relativo a la linea de proyeccion de la camara para evitar dibujar mas caras de alas necesarias.

# Dot Prouct

Es la escala de un valor simple que nos dice que tan similares son dos vectores.
D = Ax * Bx + Ay * By + Az * Bz
A = (1,0,0)
B = (0,1,0)
= 0 (Not similar)

--

A = (1,0,0)
B = (1,0,0)
= 1 (Similar)

--