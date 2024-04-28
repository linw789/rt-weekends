# Snell's Law

[<img src="images/snell-law.jpg" width="400"/>](images/snell-law.jpg)

To calculate refracted ray $R^\prime = R^\prime_\perp + R^\prime_\parallel$:

$$
R_\parallel = (R \cdot n)n \\[3pt]
R_\perp = R - R_\parallel \\[3pt]
R_\perp^\prime = \dfrac{sin\theta^\prime |R^\prime|}{|R_\perp|} R_\perp \\[3pt]
\because |R^\prime| = |R| \\[3pt]
R^\prime_\perp = \dfrac{sin\theta^\prime |R^|}{|R_\perp|} R_\perp \\[3pt]
R^\prime_\perp = \dfrac{sin\theta^\prime}{sin\theta} R_\perp \\[3pt]
R^\prime_\perp = \dfrac{\eta}{\eta^\prime} (R - (R \cdot n)n) \\[3pt]
\because |R^\prime_\parallel|^2 = |R^\prime|^2 - |R^\prime_\perp|^2 \land |R^\prime| = |R| \\[3pt]
R^\prime_\parallel = \sqrt{|R|^2 - |R^\prime_\perp|^2} (-n)
$$

# Strange Circular Banding

[<img src="images/strange-circular-banding.png" width="400"/>](images/strange-circular-banding.png)

The rendered image is also very dark. 

```rust
let limits = 0.0..Fp::MAX;
let intersection = sphere.ray_intercept(ray, &limits);
```

Because I set the lower limit to 0, `ray_intercept()` would return hit when `t` is 0. This causes
a sphere to intersect with a ray bounced from the sphere itself. This would lead to infinite
loop of successful intersection test on the sphere and its bounced ray, if the trace depth is not 
capped. Because the trace depth is capped, the trace would eventually return zero, abandoning the 
contribution of the original ray. That's why the rendered image is dark. I'm not exactly sure why
the circular banding, which is also fixed by setting the lower limits to a positive non-zero
value.
