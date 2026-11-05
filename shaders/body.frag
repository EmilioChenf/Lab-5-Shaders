#version 100
precision highp float;
varying vec3 local;
varying vec3 normalWorld;
varying vec3 world;
uniform float Time;
uniform float Kind;
uniform vec3 Eye;
float hash(vec3 p) { return fract(sin(dot(p,vec3(127.1,311.7,74.7)))*43758.5453); }
float noise(vec3 p) {
    vec3 i=floor(p),f=fract(p); f=f*f*(3.-2.*f);
    return mix(mix(mix(hash(i),hash(i+vec3(1,0,0)),f.x),mix(hash(i+vec3(0,1,0)),hash(i+vec3(1,1,0)),f.x),f.y),mix(mix(hash(i+vec3(0,0,1)),hash(i+vec3(1,0,1)),f.x),mix(hash(i+vec3(0,1,1)),hash(i+vec3(1,1,1)),f.x),f.y),f.z);
}
float fbm(vec3 p) { float n=0.,a=.5; for(int i=0;i<5;i++){n+=a*noise(p);p=p*2.03+7.1;a*=.5;}return n; }
void main() {
    vec3 p=normalize(local);
    float n=fbm(p*5.);
    float detail=fbm(p*28.);
    float rim=pow(1.-max(dot(normalize(normalWorld),normalize(Eye-world)),0.),3.);
    vec3 color=vec3(.5);
    float emission=0.;
    // BODY_LAYERS
    float light=.23+.77*max(dot(normalize(normalWorld),normalize(vec3(-.5,.8,1.))),0.);
    gl_FragColor=vec4(color*mix(light,1.,emission),1.);
}
