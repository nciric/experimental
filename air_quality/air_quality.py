''' Get reading from Nova PM Sensor SDS011 (dust sensor, air quality sensor, PM10, PM2,5)
'''
from statistics import mean

import argparse
import time
try:
    import serial
except ImportError:
    print('Python serial library required, on Ubuntu/Debian:')
    print('    apt-get install python3-serial')
    print('on Windows:')
    print('    pip install pyserial')
    raise

def read_nova_dust_sensor(sensor):
    msg = sensor.read(10)
    assert msg[0] == ord(b'\xaa')
    assert msg[1] == ord(b'\xc0')
    assert msg[9] == ord(b'\xab')
    pm25 = (msg[3] * 256 + msg[2]) / 10.0
    pm10 = (msg[5] * 256 + msg[4]) / 10.0
    checksum = sum(v for v in msg[2:8]) % 256
    assert checksum == msg[8]
    return {'PM10': pm10, 'PM2_5': pm25}

def get_aqi_2_5(reading):
    if 0.0 <= reading and reading <= 12.5:
        return get_aqi(reading, 0.0, 12.0, 0, 50)
    elif 12.1 <= reading and reading <= 35.4:
        return get_aqi(reading, 12.1, 35.4, 51, 100)
    elif 35.5 <= reading and reading <= 55.4:
        return get_aqi(reading, 35.5, 55.4, 101, 150)
    elif 55.5 <= reading and reading <= 150.4:
        return get_aqi(reading, 55.5, 150.4, 151, 200)
    elif 150.5 <= reading and reading <= 250.4:
        return get_aqi(reading, 150.5, 250.4, 201, 300)
    elif 250.5 <= reading and reading <= 350.4:
        return get_aqi(reading, 250.5, 350.4, 301, 400)
    elif 350.5 <= reading and reading <= 500.4:
        return get_aqi(reading, 350.5, 500.4, 401, 500)
    else:
        return 666 # Armageddon

def get_aqi_10_0(reading):
    if 0.0 <= reading and reading <= 54.0:
        return get_aqi(reading, 0.0, 54.0, 0, 50)
    elif 54.1 <= reading and reading <= 154.0:
        return get_aqi(reading, 54.1, 154.0, 51, 100)
    elif 154.1 <= reading and reading <= 254.0:
        return get_aqi(reading, 154.1, 254.0, 101, 150)
    elif 254.1 <= reading and reading <= 354.0:
        return get_aqi(reading, 254.1, 354.0, 151, 200)
    elif 354.1 <= reading and reading <= 424.0:
        return get_aqi(reading, 354.1, 424.0, 201, 300)
    elif 424.1 <= reading and reading <= 504.0:
        return get_aqi(reading, 424.1, 504.0, 301, 400)
    elif 504.1 <= reading and reading <= 604.0:
        return get_aqi(reading, 504.1, 604.0, 401, 500)
    else:
        return 666 # Armageddon

def get_aqi(reading, c_low, c_high, i_low, i_high):
    '''Formula from https://en.wikipedia.org/wiki/Air_quality_index#Indices_by_location, US region.'''
    return (i_high - i_low) * (reading - c_low) / (c_high - c_low) + i_low

parser = argparse.ArgumentParser()
parser.add_argument('-p', '--port', help = 'Port to listen to. Use one of COMx on Windows.', default = '/dev/ttyUSB0')
args = parser.parse_args()
sensor = serial.Serial(args.port, 9600)
if not sensor.isOpen():
    sensor.open()

pm10_queue = []
pm2_5_queue = []
max_queue_length = 20
while True:
    value = read_nova_dust_sensor(sensor)
    pm10_queue.append(value['PM10'])
    pm2_5_queue.append(value['PM2_5'])
    if len(pm10_queue) > max_queue_length:
        pm10_queue.pop(0)
        pm2_5_queue.pop(0)
    air_quality = max(get_aqi_2_5(mean(pm2_5_queue)), get_aqi_10_0(mean(pm10_queue)))
    print('Air quality: ', int(air_quality), ' PM10:', value['PM10'], ' PM2.5:', value['PM2_5'], end='\r', flush=True)
    time.sleep(1)
sensor.close()