import serial

client = serial.Serial("COM8", 115200, timeout=1) #使用USB连接串行口

while True:
  s = client.read()
  print("Recv:", s)
