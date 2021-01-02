WIDTH = 7
HEIGHT = 7

RAD = 1
OFFSET = 2.25

TOTAL_WIDTH = OFFSET * (WIDTH + 1)
START_X_OFFSET =  SCREEN_X / 2 - TOTAL_WIDTH / 2

for row = 1,HEIGHT do
    for col = 1,WIDTH do
        color = {
            r = (row * col) / (WIDTH * HEIGHT) * 255,
            g = col / WIDTH * 255,
            b = row / HEIGHT * 255
        }
        add_shape{
            shape = "circle",
            x = col * OFFSET + START_X_OFFSET,
            y = row * OFFSET,
            r = RAD,
            mass = 1,
            color = color
        }
    end
end

-- floor
add_shape{
    shape = "rect",
    status = "static",
    x = 0,
    y = SCREEN_Y,
    w = SCREEN_X,
    h = 0.25
}
