rad = 1

vA = 3.0
vB = 0.0

elasticityA = 1
elasticityB = 1

add_shapes(
    --mA
    {shape = "circle", x = rad + 1, y = SCREEN_Y / 2, r = 1.0, mass = 1, elasticity = elasticityA, x_vel = vA},

    --mB
    {shape = "circle", x = SCREEN_X / 2, y = SCREEN_Y / 2, r = 1.0, mass = 1, elasticity = elasticityB, x_vel = vB}
)

GRAVITY = 0.0
PAUSED = true

print(SCREEN_X)
