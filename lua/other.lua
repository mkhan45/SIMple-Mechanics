add_shapes(
    --floor
   {shape = "rect", x = 0, y = SCREEN_Y, status = "static", w = SCREEN_X * 5.0, h = 0.25, elasticity = 0.1}
)


tick = 0
width = 62

GRAVITY = 9.81

function update()
   if OBJECTS ~= nil then
      for k, v in pairs(OBJECTS) do
         print(v:id())
      end
   end
   if (tick % 60 == 0 and tick > 0) then
      print(string.format("FPS: %s, Mouse Pos: %.4f, %.4f", FPS, MOUSE_X, MOUSE_Y))
   end

   if (tick % 90 == 0 and tick < 90 * 6) then
      for i = 1, width do
         add_shape({shape = "circle", x = 0.485 * i, y = 10 - tick / 60, r = 0.225, elasticity = 0.99,
         color = {r = tick % 255, g = 255 - (tick % 255), b = math.floor(i / width * 255)}})
      end
   end
   tick = tick + 1
   ADD_SHAPES = true
end
