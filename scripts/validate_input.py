import ctypes as c, os, subprocess, time, shutil
from pathlib import Path
from PIL import Image, ImageChops
os.environ['DISPLAY']=':99'
x=c.CDLL('libX11.so.6'); xt=c.CDLL('libXtst.so.6')
x.XOpenDisplay.restype=c.c_void_p
x.XOpenDisplay.argtypes=[c.c_char_p]
d=x.XOpenDisplay(None)
assert d
x.XDefaultRootWindow.argtypes=[c.c_void_p];x.XDefaultRootWindow.restype=c.c_ulong
x.XQueryTree.argtypes=[c.c_void_p,c.c_ulong,c.POINTER(c.c_ulong),c.POINTER(c.c_ulong),c.POINTER(c.POINTER(c.c_ulong)),c.POINTER(c.c_uint)]
x.XFetchName.argtypes=[c.c_void_p,c.c_ulong,c.POINTER(c.c_char_p)]
x.XSetInputFocus.argtypes=[c.c_void_p,c.c_ulong,c.c_int,c.c_ulong]
x.XResizeWindow.argtypes=[c.c_void_p,c.c_ulong,c.c_uint,c.c_uint]
x.XStringToKeysym.argtypes=[c.c_char_p];x.XStringToKeysym.restype=c.c_ulong
x.XKeysymToKeycode.argtypes=[c.c_void_p,c.c_ulong];x.XKeysymToKeycode.restype=c.c_uint
x.XFlush.argtypes=[c.c_void_p]
xt.XTestFakeKeyEvent.argtypes=[c.c_void_p,c.c_uint,c.c_int,c.c_ulong]
xt.XTestFakeButtonEvent.argtypes=[c.c_void_p,c.c_uint,c.c_int,c.c_ulong]
xt.XTestFakeMotionEvent.argtypes=[c.c_void_p,c.c_int,c.c_int,c.c_int,c.c_ulong]
p=subprocess.Popen(['./target/release/lab-5-shaders'],env={**os.environ,'LIBGL_ALWAYS_SOFTWARE':'1'})
try:
 time.sleep(1)
 root=x.XDefaultRootWindow(d);a=c.c_ulong();b=c.c_ulong();children=c.POINTER(c.c_ulong)();n=c.c_uint()
 x.XQueryTree(d,root,c.byref(a),c.byref(b),c.byref(children),c.byref(n))
 window=None
 for i in range(n.value):
  title=c.c_char_p();x.XFetchName(d,children[i],c.byref(title))
  if title.value and b'Laboratorio' in title.value:window=children[i]
 assert window is not None
 x.XSetInputFocus(d,window,1,0);x.XFlush(d)
 def key(k):
  code=x.XKeysymToKeycode(d,x.XStringToKeysym(k.encode()))
  xt.XTestFakeKeyEvent(d,code,1,0);x.XFlush(d);time.sleep(.1)
  xt.XTestFakeKeyEvent(d,code,0,0);x.XFlush(d);time.sleep(.15)
 def shot(name):
  path=Path('docs/screenshots')/(name+'.png');old=path.stat().st_mtime_ns if path.exists() else 0
  key('F12');time.sleep(.25)
  assert path.stat().st_mtime_ns>old
  return Image.open(path).convert('RGB').crop((300,120,1140,800))
 for i,name in enumerate(['PAPATIN','PAPATINSENIOR','MAMBO','CRYON','CITOPA','VERDANTIS','NYX'],1):
  key(str(i));shot(name)
 print('PASS: keys 1-7 select correct screenshot names; F12 writes new frames',flush=True)
 key('6');key('space');a=shot('VERDANTIS');time.sleep(.4);b=shot('VERDANTIS')
 assert ImageChops.difference(a,b).getbbox() is None
 key('space');time.sleep(.4);b=shot('VERDANTIS')
 assert ImageChops.difference(a,b).getbbox() is not None
 print('PASS: pause freezes body pixels; resume changes them',flush=True)
 key('space');a=shot('VERDANTIS')
 xt.XTestFakeMotionEvent(d,-1,600,450,0);x.XFlush(d);time.sleep(.2)
 xt.XTestFakeButtonEvent(d,1,1,0);x.XFlush(d);time.sleep(.2)
 for step in range(1,6):
  xt.XTestFakeMotionEvent(d,-1,600+step*30,450+step*12,0);x.XFlush(d);time.sleep(.1)
 xt.XTestFakeButtonEvent(d,1,0,0);x.XFlush(d);time.sleep(.2)
 b=shot('VERDANTIS');assert ImageChops.difference(a,b).getbbox() is not None
 key('r');b=shot('VERDANTIS');assert ImageChops.difference(a,b).getbbox() is None
 xt.XTestFakeButtonEvent(d,4,1,0);xt.XTestFakeButtonEvent(d,4,0,0);x.XFlush(d);time.sleep(.2)
 b=shot('VERDANTIS');assert ImageChops.difference(a,b).getbbox() is not None
 print('PASS: mouse orbit, R reset, wheel zoom change expected image',flush=True)
 evidence=Path('docs/audit/views');evidence.mkdir(parents=True,exist_ok=True)
 key('3');shot('MAMBO');shutil.copyfile('docs/screenshots/MAMBO.png',evidence/'MAMBO_frontal.png')
 for label,start,end in [('superior',750,150),('inferior',150,750)]:
  key('r');xt.XTestFakeMotionEvent(d,-1,700,start,0);x.XFlush(d);time.sleep(.2)
  xt.XTestFakeButtonEvent(d,1,1,0);x.XFlush(d);time.sleep(.2)
  for step in range(1,6):
   xt.XTestFakeMotionEvent(d,-1,700,int(start+(end-start)*step/5),0);x.XFlush(d);time.sleep(.1)
  xt.XTestFakeButtonEvent(d,1,0,0);x.XFlush(d);time.sleep(.2)
  shot('MAMBO');shutil.copyfile('docs/screenshots/MAMBO.png',evidence/f'MAMBO_{label}.png')
  with Image.open('docs/screenshots/MAMBO.png') as image:
   image=image.convert('RGB');gold=0
   for y in range(100,850):
    for col in range(150,1290):
     r,g,b=image.getpixel((col,y))
     if (col-720)**2+(y-450)**2>240**2 and r>g*1.05 and g>b*1.1 and g>45:gold+=1
   assert gold>1000,(label,gold)
  print(f'PASS: anillos visibles desde vista {label}: {gold} píxeles dorados fuera de la esfera',flush=True)
 key('7');x.XResizeWindow(d,window,1280,720);x.XFlush(d);time.sleep(.5);shot('NYX')
 with Image.open('docs/screenshots/NYX.png') as image:assert image.size==(1280,720)
 shutil.copyfile('docs/screenshots/NYX.png',evidence/'NYX_1280x720.png')
 print('PASS: redimensionamiento real a 1280x720 y captura de Nyx',flush=True)
 x.XResizeWindow(d,window,1440,900);x.XFlush(d);time.sleep(.5)
 key('0');shot('atlas');key('Escape');p.wait(timeout=5);assert p.returncode==0
 print('PASS: 0 restores atlas; Escape exits cleanly',flush=True)
finally:
 if p.poll() is None:p.terminate();p.wait()
