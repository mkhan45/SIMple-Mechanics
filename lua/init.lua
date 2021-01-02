WIDTH = 9
box_size = 0.5

incr = box_size * 4.20 -- not 100% sure why but smaller box sizes mean the multiplier has to increase
x_offset = (SCREEN_X / 2) - ((WIDTH - 2) * incr / 2)
y_offset = 0.1 -- larger width means higher offset

max_x = x_offset + (WIDTH - 3) * incr

for row = 1,WIDTH - 1 do
    for col = WIDTH - row - 2,(WIDTH - 3)/2 do
        color = {r = 255 - row / (WIDTH - 1) * 255, g = math.max(col, 0) / ((WIDTH - 3) / 2) * 255, b = row / (WIDTH - 1) * 255}
        add_shape({shape="rect", x = x_offset + col * incr, y = row * incr - y_offset, w = 1, h = 1, mass = 1, color = color})

        if col ~= (WIDTH - 3) / 2 then
            add_shape({shape="rect", x = max_x - col * incr, y = row * incr - y_offset, w = 1, h = 1, mass = 1, color = color})
        end
    end
end

add_shape({shape = "rect", x = 0, y = SCREEN_Y - 1, w = SCREEN_X * 2, h = 1, mass = 1, status = "static"})

GRAVITY = 9.81
