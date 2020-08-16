width = 9
box_size = 0.5

incr = box_size * 4.20 -- not 100% sure why but smaller box sizes mean the multiplier has to increase
x_offset = (SCREEN_X / 2) - ((width - 2) * incr / 2)
y_offset = 0.1 -- larger width means higher offset

max_x = x_offset + (width - 3) * incr

for row = 1,width - 1 do
    for col = width - row - 2,(width - 3)/2 do
        color = {r = 255 - row / (width - 1) * 255, g = math.max(col, 0) / ((width - 3) / 2) * 255, b = row / (width - 1) * 255}
        add_shape({shape="rect", x = x_offset + col * incr, y = row * incr - y_offset, w = 1, h = 1, mass = 1, color = color})

        if col ~= (width - 3) / 2 then
            add_shape({shape="rect", x = max_x - col * incr, y = row * incr - y_offset, w = 1, h = 1, mass = 1, color = color})
        end
    end
end

add_shape({shape = "rect", x = 0, y = SCREEN_Y - 1, w = SCREEN_X * 2, h = 1, mass = 1, status = "static"})

GRAVITY = 9.81
