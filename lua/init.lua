add_shapes(
   -- floor
   --{shape = "rect", x = 0, y = SCREEN_Y, status = "static", w = SCREEN_X * 5.0, h = 0.25, elasticity = 0.1},

   -- {shape = "rect", x = 5.0, y = 12.0, w = 0.5, h = 1.25, mass = 5.0, y_vel = -15.0, rotvel = 5.0},
   -- {shape = "rect", x = 10.0, y = 0.0, w = 2.0, h = 1.0, mass = 8.0, rotation = PI / 3.0},
   -- {shape = "circle", x = 19.25, y = 1.0, r = 2.0, mass = 12.5, elasticity = 0.5, name = "circle"}

   -- static floating square
   -- {shape = "rect", x = SCREEN_X / 3.0, y = SCREEN_Y / 2.0, status = "static", w = 0.1, h = 0.1}
)

add_shapes(
    {shape = "circle", x = 5, y = SCREEN_Y / 2, r = 1.5, elasticity = 1.0, mass = 10, x_vel = 8},
    {shape = "circle", x = SCREEN_X, y = SCREEN_Y / 2, r = 1.5, elasticity = 1.0, mass = 5, x_vel = -8},

    {shape = "rect", x = 0, y = 0, w = 0.5, h = SCREEN_Y, elasticity = 0.5, mass = 5, status="static"},
    {shape = "rect", x = SCREEN_X + 5, y = 0, w = 0.5, h = SCREEN_Y, elasticity = 0.5, mass = 5, status="static"},

    {shape = "rect", x = SCREEN_X - 10, y = 4, w = 1, h = 1, elasticity = 0.5, mass = 5},
    {shape = "rect", x = 0, y = SCREEN_Y, w = SCREEN_X + 50, h = 0.5, elasticity = 0.5, mass = 1, status = "static"}
)

GRAVITY = 0

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

tick = 0
width = 62

function update()
   -- if OBJECTS ~= nil then
   --    for k, v in pairs(OBJECTS) do
   --       print(v:id())
   --    end
   -- end
   -- if (tick % 60 == 0 and tick > 0) then
   --    print(string.format("FPS: %s, Mouse Pos: %.4f, %.4f", FPS, MOUSE_X, MOUSE_Y))
   -- end

   -- if (tick % 90 == 0 and tick < 90 * 10) then
   --    for i = 1, width do
   --       add_shape({shape = "circle", x = 0.425 * i, y = 10 - tick / 60, r = 0.2, elasticity = 0.99,
   --       color = {r = tick % 255, g = 255 - (tick % 255), b = math.floor(i / width * 255)}})
   --    end
   -- end
   -- tick = tick + 1
   -- ADD_SHAPES = true
end
