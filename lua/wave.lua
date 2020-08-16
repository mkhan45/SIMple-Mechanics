add_shapes(
    {shape = "rect", status = "static", x = 0, y = SCREEN_Y, w = SCREEN_X, h = 1, mass = 1, elasticity = 0.99}
)

cols = 10

-- vf = a * t
-- p = v0*t + (a * t^2 / 2)
-- t = sqrt(p - v0*t - a/2)

-- The first and last ones are wrong somehow so I just removed them
for col = 2,(cols-1) do
    x = col / (cols + 1) * SCREEN_X
    y = col / (cols + 1) * SCREEN_Y

    grav = GRAVITY * 0.345 -- I'm not sure what the multiplier should be but lua gravity is not the same as real gravity

    y_offset = y - (1 / (cols + 1) * SCREEN_Y)
    t = math.sqrt(y_offset * grav / 2)
    y_vel = grav * t

    color = {r = 255 - col / (cols - 1) * 255, g = col / (cols - 1) * 255, b = col / (cols - 1) * 255}

    add_shape({shape = "circle", x = x, y = y, r = 1, mass = 1, elasticity = 1, y_vel = y_vel, color = color})
end
