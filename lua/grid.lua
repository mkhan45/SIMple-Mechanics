width = 7
height = 7

rad = 1
offset = 2.25

total_width = offset * (width + 1)
start_x_offset =  SCREEN_X / 2 - total_width / 2

for row = 1,height do
    for col = 1,width do
        color = {
            r = (row * col) / (width * height) * 255,
            g = col / width * 255,
            b = row / height * 255
        }
        add_shape(
            {shape = "circle", x = col * offset + start_x_offset, y = row * offset, r = rad, mass = 1, color = color}
        )
    end
end

add_shape({shape = "rect", status = "static", x = 0, y = SCREEN_Y, w = SCREEN_X, h = 0.25})
