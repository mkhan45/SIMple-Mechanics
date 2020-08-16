c1 = {r = 255, g = 100, b = 255}
c2 = {r = 100, g = 255, b = 255}
c3 = {r = 255, g = 255, b = 100}

add_shapes(
    {shape = "rect", status = "static", x = 0, y = SCREEN_Y, w = SCREEN_X, h = 1, mass = 1, elasticity = 0.8},
    {shape = "circle", x = SCREEN_X / 6, y = SCREEN_Y / 3, r = 1.5, mass = 1, elasticity = 0.5, color = c1},
    {shape = "circle", x = SCREEN_X / 2, y = SCREEN_Y / 3, r = 1.5, mass = 1, elasticity = 0.7, color = c2},
    {shape = "circle", x = SCREEN_X * 5/6, y = SCREEN_Y / 3, r = 1.5, mass = 1, elasticity = 0.9, color = c3}
)
