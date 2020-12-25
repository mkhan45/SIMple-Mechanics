offset = math.sin(math.pi / 4) * 1.5

add_shapes(
    {shape = "rect", status = "static", x = SCREEN_X / 8, y = SCREEN_Y / 6, 
     w = 3, h = 0.25, rotation = -math.pi / 4, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = SCREEN_X / 8, y = 5 * SCREEN_Y / 6,
     w = 3, h = 0.25, rotation = math.pi / 4, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = 7 * SCREEN_X / 8, y = 5 * SCREEN_Y / 6, 
     w = 3, h = 0.25, rotation = -math.pi / 4, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = 7 * SCREEN_X / 8, y = SCREEN_Y / 6, 
     w = 3, h = 0.25, rotation = math.pi / 4, mass = 1, elasticity = 1, friction = 0},

     {shape = "circle", x = SCREEN_X / 2, y = SCREEN_Y / 6, 
     r = 1, mass = 1, elasticity = 1, x_vel = 12, friction = 0},

     {shape = "circle", x = SCREEN_X / 2, y = 5 * SCREEN_Y / 6, 
     r = 1, mass = 1, elasticity = 1, x_vel = 12, friction = 0}
)

add_shapes(
    {shape = "rect", status = "static", x = SCREEN_X / 8 - offset * 2, y = SCREEN_Y / 2, 
    w = 0.25, h = SCREEN_Y / 4 - 0.25, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = 7 * SCREEN_X / 8 + offset * 2, y = SCREEN_Y / 2, 
    w = 0.25, h = SCREEN_Y / 4 - 0.25, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = SCREEN_X / 2, y = SCREEN_Y / 8 - offset, 
    w = SCREEN_X / 8, h = 0.25, mass = 1, elasticity = 1, friction = 0},

    {shape = "rect", status = "static", x = SCREEN_X / 2, y = 7 * SCREEN_Y / 8 + offset, 
    w = SCREEN_X / 8, h = 0.25, mass = 1, elasticity = 1, friction = 0},

    {shape = "circle", x = SCREEN_X / 8 - offset / 2 + 1/2, y = SCREEN_Y / 2, r = 1, 
    mass = 1, elasticity = 1, friction = 0, x_vel = 10, y_vel = 5}
)

GRAVITY = 0

function update()
end
