add_shapes(
   -- floor
   {shape = "rect", x = 0, y = SCREEN_Y, status = "static", w = SCREEN_X * 5.0, h = 0.25}

   -- {shape = "rect", x = 10.0, y = 16.0, w = 0.5, h = 1.25, mass = 5.0},
   -- {shape = "rect", x = 8.0, y = 0.0, w = 2.0, h = 1.0, mass = 8.0, rotation = PI / 3.0},
   -- {shape = "circle", x = 19.25, y = 1.0, r = 2.0, mass = 12.5, elasticity = 0.5},

   -- static floating square
   -- {shape = "rect", x = SCREEN_X / 3.0, y = SCREEN_Y / 2.0, status = "static", w = 0.1, h = 0.1}
)

tick = 0

function update()
   if (tick % 60 == 0 and tick < 60 * 10) then
      add_shape({shape = "circle", x = 12.5, y = 0, r = 0.2, elasticity = 0.999,
                 color = {r = tick % 255, g = 255 - (tick % 255), b = 255}})
   end
   tick = tick + 1
   ADD_SHAPES = true
end
