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
    // HELIOS: convective cells, dark sunspots, hot filaments, pulsing corona.
    if(Kind<.5) {
        float plasma=fbm(p*8.+vec3(Time*.13,0.,Time*.07));
        float spots=smoothstep(.60,.74,fbm(p*13.+Time*.025));
        float filaments=pow(1.-abs(sin((plasma+p.y)*22.)),7.);
        color=mix(vec3(.95,.12,.015),vec3(1.,.8,.16),plasma);
        color=mix(color,vec3(.12,.025,.008),spots*.9);
        color+=vec3(1.,.55,.12)*filaments*.45;
        color+=rim*vec3(1.,.6,.18)*(.6+.2*sin(Time*2.));
        emission=1.;
    }
    // IGNIS: basalt, ash patches, lava seams, incandescent pockets.
    else if(Kind<1.5) {
        float cracks=1.-smoothstep(.015,.07,abs(fbm(p*8.)-.49));
        float hot=smoothstep(.58,.75,detail+sin(Time+p.y*9.)*.035);
        color=mix(vec3(.025,.018,.025),vec3(.18,.09,.055),n);
        color+=vec3(.12,.10,.09)*detail;
        color=mix(color,vec3(1.,.19,.005),cracks*.95);
        color+=vec3(1.,.48,.015)*hot*.8;
        emission=max(cracks,hot)*.9;
    }
    // AURELION: cream bands, turbulent ochre belts, oval storm, fine clouds.
    else if(Kind<2.5) {
        float bands=sin(p.y*40.+fbm(p*9.+Time*.018)*5.);
        color=mix(vec3(.32,.14,.07),vec3(.95,.80,.51),bands*.5+.5);
        color=mix(color,vec3(.76,.43,.16),smoothstep(.5,.7,n)*.6);
        vec2 q=vec2(p.x-.35,(p.y+.22)*2.5);
        float storm=(1.-smoothstep(.12,.25,length(q)))*smoothstep(.25,.7,p.z);
        color=mix(color,vec3(.65,.16,.065)+.15*sin(length(q)*95.+n*12.),storm);
        color+=vec3(.16,.13,.09)*detail;
    }
    // BODY_LAYERS
    float light=.23+.77*max(dot(normalize(normalWorld),normalize(vec3(-.5,.8,1.))),0.);
    gl_FragColor=vec4(color*mix(light,1.,emission),1.);
}
