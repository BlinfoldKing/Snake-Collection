require 'gosu'

include Gosu

#dir symbol

class Snake
	def initialize
		@x = 10
		@y = 10
		@dir = :right
	end

	def getY
		return @y
	end

	def getX
		return @x
	end

	def move!
		case @dir
		when :up
			@y -= 1
		when :down
			@y += 1
		when :right
			@x += 1
		when :left
			@x -= 1
		end
	end

	def changeDir dir
		@dir = dir
	end

	def grow!
	end
end

class Game < Gosu::Window
	def initialize
    	super 500, 500
		self.caption = "Tutorial Game"
		@snake = Snake.new
	end
	  
	def update
		@snake.move!
	end
  
	def draw
		draw_rect(0, 0, 500, 500, Color::BLACK)
		draw_rect(@snake.getX * 10, @snake.getY * 10, 10, 10, Color::WHITE)
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
	end
end

Game.new.show