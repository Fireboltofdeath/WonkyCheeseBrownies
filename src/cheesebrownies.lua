local consumer = {};
consumer.__index = consumer;

function consumer.new()
	return setmetatable({
		Sauces = {}
	}, consumer);
end

function consumer:Eat()
	print("I eat brownie with sauces:", table.concat(self.Sauces, ", "));
end

function consumer:Sauce(...)
	for i = select('#', ...) do
		print(("I pour %s sauce on my brownie!"):format(select(i, ...)));
		table.insert(self.Sauces, select(i, ...));
	end
end

function consumer:Cheese()
	self:Sauce("Cheese");
end

return consumer;