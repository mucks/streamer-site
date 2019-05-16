import {Component, HostListener, OnInit} from '@angular/core';
import * as b from 'babylonjs';

@Component({
  selector: 'app-background',
  templateUrl: './background.component.html',
  styleUrls: ['./background.component.css']
})
export class BackgroundComponent implements OnInit {
  engine: b.Engine;
  constructor() {}

  ngOnInit() {}

  startScene() {
    const canvas = document.getElementById('renderCanvas') as HTMLCanvasElement;

    this.engine = new b.Engine(canvas, true);

    const scene = new b.Scene(this.engine);
    const camera = new b.FreeCamera('camera1', new b.Vector3(0, 5, -10), scene);
    camera.setTarget(b.Vector3.Zero());
    camera.attachControl(canvas, false);

    var particleSystem = new b.ParticleSystem('particles', 2000, scene);
    particleSystem.start();
    particleSystem.particleTexture = new b.Texture('assets/flare.png', scene);
    particleSystem.textureMask = new BABYLON.Color4(0.1, 0.8, 0.8, 1.0);
    particleSystem.emitter = new b.Vector3(-1, 2, 3);

    this.engine.runRenderLoop(() => {
      scene.render();
      document.body.style.background = 'url(' + canvas.toDataURL() + ')';
    });
  }

  @HostListener('window:resize', ['$event'])
  onResize(event) {
    this.engine.resize();
  }
}
