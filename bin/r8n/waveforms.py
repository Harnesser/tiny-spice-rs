

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


def load(filename, col=1):
	wv = Waveform()

	hDAT = open(filename, 'r')

	# 1st line is names
	names = hDAT.readline().strip().split()
	wv.x_name = names[0]
	wv.y_name = names[col]

	# 2nd line is units
	units = hDAT.readline().strip().split()
	wv.x_unit = units[0]
	wv.y_unit = units[col]

	# rest are datapoints
	for line in hDAT:
		data = line.strip().split()
		wv.x.append(float(data[0]))
		wv.y.append(float(data[col]))

	hDAT.close()

	return wv








