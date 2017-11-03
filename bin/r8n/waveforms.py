

class Waveform(object):
	def __init__(self):
		self.x_name = None
		self.x_unit = None
		self.x = []
		self.y_name = None
		self.y_unit = None
		self.y = []

	def summary(self):
		msg = ""
		msg += "Waveform\n"
		msg += " X-axis: {} ({}) {} points".format(self.x_name, self.x_unit, len(self.x))
		return msg


def _parse_expr(expr):
	""" Supported:
		"2" - data = column 2
		"3-2" - data = column 3 - column 2
	"""
	bits_untrimmed = expr.split('-')
	bits = [b.strip() for b in bits_untrimmed]
	if len(bits) == 3:
		return ('-', int(bits[0]), int(bits[2]))
	else:
		return ('=', int(bits[0]))


def load(filename, expr=1):
	wv = Waveform()

	op = _parse_expr(expr)

	hDAT = open(filename, 'r')

	# 1st line is names
	names = hDAT.readline().strip().split()
	wv.x_name = names[0]
	wv.y_name = names[op[1]]

	# 2nd line is units
	units = hDAT.readline().strip().split()
	wv.x_unit = units[0]
	wv.y_unit = units[op[1]]

	# rest are datapoints
	for line in hDAT:
		data = line.strip().split()
		wv.x.append(float(data[0]))
		wv.y.append(float(data[op[1]]))
		if op[0] == '-':
			wv.y[-1] -= float(data[op[2]])

	hDAT.close()

	return wv








