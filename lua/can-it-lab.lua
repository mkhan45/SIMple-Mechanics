add_shapes(
    {shape = "rect", status = "static", x = SCREEN_X / 2, y = SCREEN_Y, 
    w = SCREEN_X, h = 0.25, mass = 1, elasticity = 0, friction = 1},

    {shape = "rect", status = "static", x = 7 * SCREEN_X / 8, y = SCREEN_Y / 2, 
    w = SCREEN_X / 8, h = 0.25, rotation = math.pi / 6, mass = 1, elasticity = 0, friction = 1},

    {shape = "rect", status = "static", x = SCREEN_X - 0.5, y = SCREEN_Y / 2 - 0.25, 
    w = 0.25, h = SCREEN_Y / 8, mass = 1, elasticity = 0, friction = 1},

    {shape = "rect", status = "static", x = SCREEN_X / 10, y = SCREEN_Y - 1, w = 0.25, h = 1, mass = 4},
    {shape = "rect", status = "static", x = 2 * SCREEN_X / 10, y = SCREEN_Y - 1, w = 0.25, h = 1, mass = 4},

    {shape = "circle", x = 7.5 * SCREEN_X / 8, y = SCREEN_Y / 2 - 0.75, mass = 1, r = 0.75}
)

GRAVITY = 9.81
