import serial

client = serial.Serial("COM8", 115200, timeout=1)

while True:
  s = client.read()
  if s == b'\x7f':
    print("Send")
    client.write("123".encode("utf-8"))
  print("Recv:", s)
