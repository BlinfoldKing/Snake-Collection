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

	def eat_food? food
		return @body[0][0] == food[0] && @body[0][1] == food[1] 
	end

	def changeDir dir
		@dir = dir
	end

	def getDir
		return @dir
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
	
	def reset!
		@snake = Snake.new
		@snake.grow!
		@snake.grow!
		generateFood!
	end

	def initialize
    	super 500, 500, false, 100 # update every 100 ms
		self.caption = "Snake Game"
		reset!
	end
	
	def generateFood!
		@food = [Random.rand(49), Random.rand(49)]
	end

	def update
		@snake.move!
		if @snake.eat_food? @food
			@snake.grow!
			generateFood!
		end
	end
  
	def draw
		draw_rect(0, 0, 500, 500, Color::BLACK)
		draw_rect(@food[0] * 10, @food[1] * 10, 10, 10, Color::RED)		
		@snake.getBody.each do |body|
			draw_rect(body[0] * 10, body[1] * 10, 10, 10, Color::WHITE)
		end
	end

	def button_down id
		if id == KB_ESCAPE
			close
		elsif id == KB_DOWN
			@snake.changeDir :down unless @snake.getDir == :up
		elsif id == KB_UP
			@snake.changeDir :up unless @snake.getDir == :down
		elsif id == KB_RIGHT
			@snake.changeDir :right unless @snake.getDir == :left
		elsif id == KB_LEFT 
			@snake.changeDir :left unless @snake.getDir == :right
		end
	end
end

Game.new.show