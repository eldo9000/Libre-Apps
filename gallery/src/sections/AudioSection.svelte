<script>
  const sounds = [
    {
      id: 'AUD-1',
      label: 'Button Click',
      category: 'interaction',
      description: 'Primary action confirm — short tick, ~8ms decay',
      play: () => synth({ freq: 1200, decay: 0.08, type: 'square', gain: 0.18 }),
    },
    {
      id: 'AUD-2',
      label: 'Button Click (Secondary)',
      category: 'interaction',
      description: 'Softer tick for secondary / ghost buttons',
      play: () => synth({ freq: 900, decay: 0.06, type: 'triangle', gain: 0.12 }),
    },
    {
      id: 'AUD-3',
      label: 'Toggle On',
      category: 'interaction',
      description: 'Rising two-tone — enable / turn on',
      play: () => twoTone(600, 900, 0.07),
    },
    {
      id: 'AUD-4',
      label: 'Toggle Off',
      category: 'interaction',
      description: 'Falling two-tone — disable / turn off',
      play: () => twoTone(900, 600, 0.07),
    },
    {
      id: 'AUD-5',
      label: 'Menu Open',
      category: 'interaction',
      description: 'Soft pop when a dropdown / context menu opens',
      play: () => synth({ freq: 480, decay: 0.12, type: 'sine', gain: 0.14 }),
    },
    {
      id: 'AUD-6',
      label: 'Menu Close',
      category: 'interaction',
      description: 'Muted tap on dismiss',
      play: () => synth({ freq: 320, decay: 0.09, type: 'sine', gain: 0.10 }),
    },
    {
      id: 'AUD-7',
      label: 'Tab Switch',
      category: 'navigation',
      description: 'Subtle click on nav tab change',
      play: () => synth({ freq: 1050, decay: 0.05, type: 'square', gain: 0.10 }),
    },
    {
      id: 'AUD-8',
      label: 'Window Open',
      category: 'navigation',
      description: 'Rising sweep — new window or sheet presents',
      play: () => sweep(300, 700, 0.18),
    },
    {
      id: 'AUD-9',
      label: 'Window Close',
      category: 'navigation',
      description: 'Falling sweep — window dismissed',
      play: () => sweep(700, 300, 0.15),
    },
    {
      id: 'AUD-10',
      label: 'Success',
      category: 'feedback',
      description: 'Positive confirmation — two ascending tones',
      play: () => chime([660, 880], 0.10),
    },
    {
      id: 'AUD-11',
      label: 'Error',
      category: 'feedback',
      description: 'Low double-buzz — destructive or blocked action',
      play: () => buzz(160, 0.22),
    },
    {
      id: 'AUD-12',
      label: 'Warning',
      category: 'feedback',
      description: 'Mid-range single pulse — caution state',
      play: () => synth({ freq: 440, decay: 0.20, type: 'sawtooth', gain: 0.12 }),
    },
    {
      id: 'AUD-13',
      label: 'Notification',
      category: 'feedback',
      description: 'Gentle three-note arpeggio',
      play: () => chime([523, 659, 784], 0.09),
    },
    {
      id: 'AUD-14',
      label: 'File Drop',
      category: 'system',
      description: 'Thud on successful drag-drop',
      play: () => synth({ freq: 120, decay: 0.25, type: 'sine', gain: 0.22 }),
    },
    {
      id: 'AUD-15',
      label: 'File Drop Rejected',
      category: 'system',
      description: 'Flat buzz — invalid drop target',
      play: () => buzz(200, 0.18),
    },
    {
      id: 'AUD-16',
      label: 'Copy',
      category: 'system',
      description: 'Crisp high tick — clipboard write',
      play: () => synth({ freq: 1600, decay: 0.05, type: 'square', gain: 0.14 }),
    },
    {
      id: 'AUD-17',
      label: 'Delete',
      category: 'system',
      description: 'Low pop — item removed',
      play: () => synth({ freq: 180, decay: 0.18, type: 'sine', gain: 0.16 }),
    },
    {
      id: 'AUD-18',
      label: 'Save Complete',
      category: 'system',
      description: 'Soft two-note confirm — file written to disk',
      play: () => chime([523, 659], 0.08),
    },
  ];

  const categories = ['interaction', 'navigation', 'feedback', 'system'];

  function ctx() {
    return new (window.AudioContext || window.webkitAudioContext)();
  }

  function synth({ freq, decay, type, gain }) {
    const ac = ctx();
    const osc = ac.createOscillator();
    const env = ac.createGain();
    osc.type = type;
    osc.frequency.setValueAtTime(freq, ac.currentTime);
    env.gain.setValueAtTime(gain, ac.currentTime);
    env.gain.exponentialRampToValueAtTime(0.0001, ac.currentTime + decay);
    osc.connect(env);
    env.connect(ac.destination);
    osc.start();
    osc.stop(ac.currentTime + decay);
    osc.onended = () => ac.close();
  }

  function twoTone(f1, f2, decay) {
    const ac = ctx();
    [f1, f2].forEach((freq, i) => {
      const osc = ac.createOscillator();
      const env = ac.createGain();
      const t = ac.currentTime + i * decay * 0.6;
      osc.type = 'triangle';
      osc.frequency.setValueAtTime(freq, t);
      env.gain.setValueAtTime(0.14, t);
      env.gain.exponentialRampToValueAtTime(0.0001, t + decay);
      osc.connect(env);
      env.connect(ac.destination);
      osc.start(t);
      osc.stop(t + decay);
    });
    setTimeout(() => ac.close(), (decay * 1.6 + 0.1) * 1000);
  }

  function sweep(fStart, fEnd, duration) {
    const ac = ctx();
    const osc = ac.createOscillator();
    const env = ac.createGain();
    osc.type = 'sine';
    osc.frequency.setValueAtTime(fStart, ac.currentTime);
    osc.frequency.linearRampToValueAtTime(fEnd, ac.currentTime + duration);
    env.gain.setValueAtTime(0.14, ac.currentTime);
    env.gain.exponentialRampToValueAtTime(0.0001, ac.currentTime + duration);
    osc.connect(env);
    env.connect(ac.destination);
    osc.start();
    osc.stop(ac.currentTime + duration);
    osc.onended = () => ac.close();
  }

  function chime(freqs, decay) {
    const ac = ctx();
    freqs.forEach((freq, i) => {
      const osc = ac.createOscillator();
      const env = ac.createGain();
      const t = ac.currentTime + i * 0.09;
      osc.type = 'sine';
      osc.frequency.setValueAtTime(freq, t);
      env.gain.setValueAtTime(0.13, t);
      env.gain.exponentialRampToValueAtTime(0.0001, t + decay + 0.15);
      osc.connect(env);
      env.connect(ac.destination);
      osc.start(t);
      osc.stop(t + decay + 0.2);
    });
    setTimeout(() => ac.close(), (freqs.length * 0.09 + 0.4) * 1000);
  }

  function buzz(freq, duration) {
    const ac = ctx();
    const osc = ac.createOscillator();
    const env = ac.createGain();
    osc.type = 'sawtooth';
    osc.frequency.setValueAtTime(freq, ac.currentTime);
    env.gain.setValueAtTime(0.18, ac.currentTime);
    env.gain.setValueAtTime(0.18, ac.currentTime + duration * 0.4);
    env.gain.exponentialRampToValueAtTime(0.0001, ac.currentTime + duration);
    osc.connect(env);
    env.connect(ac.destination);
    osc.start();
    osc.stop(ac.currentTime + duration);
    osc.onended = () => ac.close();
  }

  let playing = $state(null);

  function playSound(sound) {
    playing = sound.id;
    sound.play();
    setTimeout(() => { if (playing === sound.id) playing = null; }, 400);
  }
</script>

<section class="audio-section">
  <h2>Audio</h2>
  <p class="section-desc">Synthesized UI sounds for the Libre app family. All tones generated via Web Audio API — no asset files.</p>

  {#each categories as cat}
    {@const group = sounds.filter(s => s.category === cat)}
    <div class="category-group">
      <h3 class="category-label">{cat}</h3>
      <div class="sound-grid">
        {#each group as sound}
          <button
            class="sound-row"
            class:active={playing === sound.id}
            onclick={() => playSound(sound)}
          >
            <span class="play-icon">{playing === sound.id ? '▶' : '▷'}</span>
            <span class="sound-label">{sound.label}</span>
            <span class="sound-desc">{sound.description}</span>
          </button>
        {/each}
      </div>
    </div>
  {/each}
</section>

<style>
  .audio-section {
    padding: 32px 40px 64px;
    max-width: 760px;
  }

  h2 {
    font-size: 22px;
    font-weight: 600;
    margin: 0 0 6px;
    color: var(--text-primary);
  }

  .section-desc {
    font-size: 13px;
    color: var(--text-muted);
    margin: 0 0 32px;
  }

  .category-group {
    margin-bottom: 28px;
  }

  .category-label {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-muted);
    margin: 0 0 8px;
  }

  .sound-grid {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sound-row {
    display: grid;
    grid-template-columns: 20px 180px 1fr;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 6px;
    background: transparent;
    border: 1px solid transparent;
    cursor: pointer;
    text-align: left;
    transition: background var(--duration-fast) var(--ease-out),
                border-color var(--duration-fast) var(--ease-out);
  }

  .sound-row:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
    border-color: var(--border);
  }

  .sound-row.active {
    background: color-mix(in srgb, var(--accent) 10%, var(--surface));
    border-color: color-mix(in srgb, var(--accent) 30%, transparent);
  }

  .play-icon {
    font-size: 11px;
    color: var(--accent);
    line-height: 1;
  }

  .sound-label {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .sound-desc {
    font-size: 12px;
    color: var(--text-muted);
  }
</style>
