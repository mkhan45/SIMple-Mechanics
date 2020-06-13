add_shapes(
   -- floor
   {shape = "rect", x = 0, y = SCREEN_Y, status = "static", w = SCREEN_X * 5.0, h = 0.25, elasticity = 0.1},

   {shape = "rect", x = 10.0, y = 16.0, w = 0.5, h = 1.25, mass = 5.0},
   {shape = "rect", x = 8.0, y = 0.0, w = 2.0, h = 1.0, mass = 8.0, rotation = PI / 3.0},
   {shape = "circle", x = 19.25, y = 1.0, r = 2.0, mass = 12.5, elasticity = 0.5},

   -- static floating square
   {shape = "rect", x = SCREEN_X / 3.0, y = SCREEN_Y / 2.0, status = "static", w = 0.1, h = 0.1}
)

for i = 1, 13 do
   for j = 1, 20 do
      add_shape({
         shape = "rect", 
         x = i * ((SCREEN_X * 0.9) / 10), 
         y = j * ((SCREEN_Y * 0.9) / 20), 
         w = 0.5, 
         h = 0.5, 
         r = 0.5,
         elasticity = 0.05, 
         friction = 1.0
      })
   end
end

function square(x)
   return x * x
end

function rotated_rect(x1, y1, x2, y2)
   local x_comp, y_comp = x2 - x1, y2 - y1
   local len = math.sqrt(square(x_comp) + square(y_comp))
   local angle = math.atan((y_comp) / (x_comp))

   return {
      shape = "rect",
      status = "static",
      x = x1 + (x_comp / 2),
      y = y1 + (y_comp / 2),
      w = len / 2,
      h = 0.1,
      rotation = angle,
   }
end

function eqn(x)
   return SCREEN_Y - square(x / 5)
end

function graph(eqn, start_x, end_x, steps, x_off, y_off)
   for i = 1, steps do
      x1 = i / steps * (end_x - start_x) + start_x
      y1 = eqn(x1)
      x2 = (i + 1) / steps * (end_x - start_x) + start_x
      y2 = eqn(x2)
      add_shape(rotated_rect(x1 + x_off, y1 + y_off, x2, y2))
   end
end

graph(eqn, -5, 5, 20, 1, 0)
