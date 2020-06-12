add_shape({shape = "rect", x = 10.0, y = 16.0, w = 1.0, h = 1.0, mass = 5.0})
add_shape({shape = "rect", x = 8.0, y = 0.0, w = 2.0, h = 1.0, mass = 8.0, rotation = PI / 3.0})
add_shape({shape = "circle", x = 15.25, y = 1.0, r = 2.0, mass = 12.5, elasticity = 0.5})

add_shape({shape = "rect", x = 0, y = SCREEN_Y, status = "static", w = SCREEN_X * 5.0, h = 0.25, elasticity = 0.5})
add_shape({shape = "rect", x = SCREEN_X / 3.0, y = SCREEN_Y / 2.0, status = "static", w = 0.1, h = 0.1, elasticity = 0.5})
