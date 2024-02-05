import serial
import sys

command = sys.argv[1]
ser = serial.Serial("/dev/cu.usbmodemTEST1", 9600)
ser.write(bytearray(f'{command}\0','ascii'))

bs = ser.readline()
print (bs.decode('ascii'))
