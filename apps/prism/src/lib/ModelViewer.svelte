<script>
  import { onMount, onDestroy } from 'svelte';
  import * as THREE from 'three';
  import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
  import { OBJLoader } from 'three/addons/loaders/OBJLoader.js';
  import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
  import { STLLoader } from 'three/addons/loaders/STLLoader.js';

  let { path, ext, streamUrl } = $props();

  let canvasEl = $state(null);
  let loadState = $state('loading'); // 'loading' | 'ready' | 'error'
  let errorMsg = $state('');
  let wireframe = $state(false);

  let renderer = null;
  let scene = null;
  let camera = null;
  let controls = null;
  let animFrameId = null;
  let resizeObserver = null;
  let loadedObject = null;
  let initialCameraPos = null;
  let initialTarget = null;

  function fitCameraToModel(obj) {
    const box = new THREE.Box3().setFromObject(obj);
    const center = new THREE.Vector3();
    box.getCenter(center);
    const size = new THREE.Vector3();
    box.getSize(size);

    camera.position.set(
      center.x,
      center.y,
      center.z + size.length() * 1.5
    );
    controls.target.copy(center);
    controls.update();

    initialCameraPos = camera.position.clone();
    initialTarget = controls.target.clone();
  }

  function resetView() {
    if (!initialCameraPos || !initialTarget) return;
    camera.position.copy(initialCameraPos);
    controls.target.copy(initialTarget);
    controls.update();
  }

  function toggleWireframe() {
    wireframe = !wireframe;
    if (loadedObject) {
      loadedObject.traverse((child) => {
        if (child.isMesh && child.material) {
          if (Array.isArray(child.material)) {
            child.material.forEach(m => { m.wireframe = wireframe; });
          } else {
            child.material.wireframe = wireframe;
          }
        }
      });
    }
  }

  function animate() {
    animFrameId = requestAnimationFrame(animate);
    controls.update();
    renderer.render(scene, camera);
  }

  onMount(() => {
    if (!canvasEl) return;

    const w = canvasEl.parentElement.clientWidth;
    const h = canvasEl.parentElement.clientHeight;

    // Scene
    scene = new THREE.Scene();
    const bgColor = getComputedStyle(document.documentElement).getPropertyValue('--surface').trim() || '#111827';
    scene.background = new THREE.Color(bgColor);

    // Camera
    camera = new THREE.PerspectiveCamera(60, w / h, 0.01, 1000);
    camera.position.set(0, 0, 5);

    // Renderer
    renderer = new THREE.WebGLRenderer({ canvas: canvasEl, antialias: true });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(w, h);

    // Lights
    const ambient = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambient);
    const dirLight = new THREE.DirectionalLight(0xffffff, 1.0);
    dirLight.position.set(5, 10, 5);
    scene.add(dirLight);

    // Controls
    controls = new OrbitControls(camera, canvasEl);
    controls.autoRotate = false;
    controls.enableDamping = true;
    controls.dampingFactor = 0.08;

    // Load model
    const url = streamUrl(path);
    const lowerExt = ext ? ext.toLowerCase() : '';

    function onLoaded(obj) {
      // Normalize: some loaders return a scene/group, others a mesh
      loadedObject = obj;
      scene.add(obj);
      fitCameraToModel(obj);
      loadState = 'ready';
      animate();
    }

    function onError(e) {
      errorMsg = String(e?.message || e);
      loadState = 'error';
    }

    try {
      if (lowerExt === 'obj') {
        const loader = new OBJLoader();
        loader.load(url, onLoaded, undefined, onError);
      } else if (lowerExt === 'gltf' || lowerExt === 'glb') {
        const loader = new GLTFLoader();
        loader.load(url, (gltf) => onLoaded(gltf.scene), undefined, onError);
      } else if (lowerExt === 'stl') {
        const loader = new STLLoader();
        loader.load(url, (geometry) => {
          const material = new THREE.MeshPhongMaterial({ color: 0x888888, specular: 0x333333 });
          const mesh = new THREE.Mesh(geometry, material);
          onLoaded(mesh);
        }, undefined, onError);
      } else {
        // Fallback: try GLTFLoader for fbx/ply/3ds — may fail but worth trying
        const loader = new GLTFLoader();
        loader.load(url, (gltf) => onLoaded(gltf.scene), undefined, (e) => {
          errorMsg = `Unsupported 3D format: .${lowerExt}`;
          loadState = 'error';
        });
      }
    } catch (e) {
      onError(e);
    }

    // Resize observer
    resizeObserver = new ResizeObserver(() => {
      if (!canvasEl || !canvasEl.parentElement) return;
      const w2 = canvasEl.parentElement.clientWidth;
      const h2 = canvasEl.parentElement.clientHeight;
      camera.aspect = w2 / h2;
      camera.updateProjectionMatrix();
      renderer.setSize(w2, h2);
    });
    resizeObserver.observe(canvasEl.parentElement);
  });

  onDestroy(() => {
    if (animFrameId) cancelAnimationFrame(animFrameId);
    if (resizeObserver) resizeObserver.disconnect();
    if (renderer) renderer.dispose();
  });
</script>

<div class="flex flex-col h-full bg-[var(--surface)]">
  <!-- 3D canvas area -->
  <div class="flex-1 min-h-0 relative">
    <canvas bind:this={canvasEl} class="w-full h-full block"></canvas>

    {#if loadState === 'loading'}
      <div class="absolute inset-0 flex flex-col items-center justify-center gap-3 bg-[var(--surface)]">
        <div class="w-8 h-8 rounded-full border-2 border-gray-700 border-t-[var(--accent)] animate-spin"></div>
        <span class="text-gray-400 text-sm">Loading 3D model…</span>
      </div>
    {:else if loadState === 'error'}
      <div class="absolute inset-0 flex flex-col items-center justify-center gap-3 text-center bg-[var(--surface)] px-8">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor" class="text-red-400">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
        <div>
          <p class="text-red-400 font-medium">Could not load model</p>
          <p class="text-gray-500 text-sm mt-1">{errorMsg}</p>
          {#if ext}
            <span class="inline-block mt-2 px-2 py-0.5 rounded bg-gray-800 text-gray-400 text-xs uppercase tracking-wide">{ext}</span>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <!-- Bottom toolbar -->
  {#if loadState === 'ready'}
    <div class="h-10 bg-gray-800 border-t border-gray-700 flex items-center px-3 gap-2 shrink-0">
      <button
        onclick={resetView}
        class="flex items-center gap-1.5 px-2.5 h-6 rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors text-xs border border-gray-600"
        title="Reset camera view"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
          <path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/>
        </svg>
        Reset View
      </button>

      <button
        onclick={toggleWireframe}
        class="flex items-center gap-1.5 px-2.5 h-6 rounded transition-colors text-xs border {wireframe ? 'bg-[var(--accent)] text-white border-[var(--accent)]' : 'text-gray-300 hover:bg-gray-700 hover:text-white border-gray-600'}"
        title="Toggle wireframe"
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/>
        </svg>
        Wireframe
      </button>

      <div class="flex-1"></div>

      <span class="text-gray-500 text-xs truncate max-w-[220px]">
        {path.split('/').pop()}
      </span>
      {#if ext}
        <span class="px-1.5 py-0.5 rounded bg-gray-700 text-gray-400 text-[10px] uppercase tracking-wide shrink-0">{ext}</span>
      {/if}
    </div>
  {/if}
</div>
