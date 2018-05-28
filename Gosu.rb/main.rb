require 'gosu'

include Gosu

class Snake
	def initialize
		@dir = :right
		@body = [[10, 10]]
	end

	def getY
		return @body[0][1]
	end

	def getX
		return @body[0][0]
	end

	def move!
		
		(@body.length - 1).step(1, -1) do |i|
			puts i - 1
			@body[i] = @body[i - 1].dup
		end
		
		case @dir
		when :up
			@body[0][1] -= 1
		when :down
			@body[0][1] += 1
		when :right
			@body[0][0] += 1
		when :left
			@body[0][0] -= 1
		end


	end

	def changeDir dir
		@dir = dir
	end

	def grow!
		new_tail = @body.last.dup
		case @dir
		when :up
			new_tail[1] += 1
		when :down
			new_tail[1] -= 1
		when :right
			new_tail[0] -= 1
		when :left
			new_tail[0] += 1
		end

		@body.push new_tail
	end

	def getBody
		return @body
	end
end

class Game < Gosu::Window
	def initialize
    	super 500, 500
		self.caption = "Tutorial Game"
		@snake = Snake.new
		@snake.grow!
		@snake.grow!

		puts @snake.getBody
	end
	  
	def update
		# @snake.move!
	end
  
	def draw
		draw_rect(0, 0, 500, 500, Color::BLACK)
		@snake.getBody.each do |body|
			draw_rect(body[0] * 10, body[1] * 10, 10, 10, Color::WHITE)
		end
	end

	def button_down id
		if id == KB_ESCAPE
			close
		elsif id == KB_DOWN
			@snake.changeDir :down
		elsif id == KB_UP
			@snake.changeDir :up
		elsif id == KB_RIGHT
			@snake.changeDir :right
		elsif id == KB_LEFT 
			@snake.changeDir :left 
		end
		@snake.move!
	end
end

Game.new.show