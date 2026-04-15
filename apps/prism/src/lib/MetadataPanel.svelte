<script>
  let { metadata, onClose } = $props();

  function formatSize(bytes) {
    if (!bytes) return '—';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }

  function formatBitrate(bps) {
    if (!bps) return '—';
    if (bps < 1000) return `${bps} bps`;
    if (bps < 1_000_000) return `${(bps / 1000).toFixed(0)} kbps`;
    return `${(bps / 1_000_000).toFixed(1)} Mbps`;
  }

  function formatDuration(secs) {
    if (secs == null) return '—';
    const s = Math.floor(secs);
    const h = Math.floor(s / 3600);
    const m = Math.floor((s % 3600) / 60);
    const sec = s % 60;
    if (h > 0) return `${h}h ${m}m ${sec}s`;
    if (m > 0) return `${m}m ${sec}s`;
    return `${sec}s`;
  }

  async function copyValue(val) {
    if (!val || val === '—') return;
    try {
      await navigator.clipboard.writeText(String(val));
    } catch {
      // clipboard access may fail in sandboxed contexts
    }
  }

  // Build key/value rows from metadata
  let rows = $derived.by(() => {
    if (!metadata) return [];
    const r = [];
    const add = (label, value) => {
      if (value != null && value !== '' && value !== '—') {
        r.push({ label, value: String(value) });
      }
    };

    add('File size', formatSize(metadata.file_size));

    // Image
    if (metadata.width && metadata.height) {
      add('Resolution', `${metadata.width} × ${metadata.height} px`);
    }
    if (metadata.color_space) add('Color space', metadata.color_space);
    if (metadata.bit_depth)   add('Bit depth', `${metadata.bit_depth}-bit`);
    if (metadata.exif_camera_make)  add('Camera make',  metadata.exif_camera_make);
    if (metadata.exif_camera_model) add('Camera model', metadata.exif_camera_model);
    if (metadata.exif_lens)         add('Lens',         metadata.exif_lens);
    if (metadata.exif_date_taken)   add('Date taken',   metadata.exif_date_taken);
    if (metadata.exif_gps_lat)      add('GPS latitude', metadata.exif_gps_lat);
    if (metadata.exif_gps_lng)      add('GPS longitude',metadata.exif_gps_lng);

    // Video / Audio
    if (metadata.codec)         add('Codec',        metadata.codec);
    if (metadata.bitrate)       add('Bitrate',      formatBitrate(metadata.bitrate));
    if (metadata.duration_secs != null) add('Duration', formatDuration(metadata.duration_secs));
    if (metadata.frame_rate)    add('Frame rate',   metadata.frame_rate);
    if (metadata.audio_tracks)  add('Audio tracks', metadata.audio_tracks);
    if (metadata.sample_rate)   add('Sample rate',  `${metadata.sample_rate} Hz`);
    if (metadata.channels)      add('Channels',     metadata.channels);
    if (metadata.id3_title)     add('Title',        metadata.id3_title);
    if (metadata.id3_artist)    add('Artist',       metadata.id3_artist);
    if (metadata.id3_album)     add('Album',        metadata.id3_album);

    // PDF
    if (metadata.pdf_version)       add('PDF version',   metadata.pdf_version);
    if (metadata.pdf_page_count)    add('Pages',         metadata.pdf_page_count);
    if (metadata.pdf_title)         add('Title',         metadata.pdf_title);
    if (metadata.pdf_author)        add('Author',        metadata.pdf_author);
    if (metadata.pdf_creation_date) add('Created',       metadata.pdf_creation_date);

    // 3D
    if (metadata.vertex_count)   add('Vertices',   metadata.vertex_count.toLocaleString());
    if (metadata.face_count)     add('Faces',      metadata.face_count.toLocaleString());
    if (metadata.material_count) add('Materials',  metadata.material_count);

    return r;
  });

  let copiedLabel = $state('');

  async function handleCopy(label, value) {
    await copyValue(value);
    copiedLabel = label;
    setTimeout(() => { copiedLabel = ''; }, 1200);
  }
</script>

<!-- Slide-in panel from right -->
<div
  class="absolute top-0 right-0 h-full w-72 bg-[var(--surface,#1f2937)] border-l border-[var(--border,#374151)] flex flex-col shadow-2xl z-20"
  style="transition: transform 120ms ease;"
>
  <!-- Header -->
  <div class="flex items-center justify-between px-4 py-3 border-b border-[var(--border,#374151)] shrink-0">
    <span class="text-[var(--text-primary,#f9fafb)] text-sm font-medium">File Info</span>
    <button
      onclick={onClose}
      class="w-6 h-6 flex items-center justify-center rounded text-[var(--text-secondary,#9ca3af)] hover:text-[var(--text-primary,#f9fafb)] hover:bg-white/10 transition-colors"
      aria-label="Close info panel"
    >
      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
        <path d="M18 6 6 18M6 6l12 12"/>
      </svg>
    </button>
  </div>

  <!-- Rows -->
  <div class="flex-1 overflow-y-auto py-2">
    {#if rows.length === 0}
      <div class="px-4 py-6 text-center text-[var(--text-secondary,#9ca3af)] text-xs">
        No metadata available
      </div>
    {:else}
      {#each rows as row}
        <button
          class="w-full flex items-start justify-between gap-3 px-4 py-2 text-left hover:bg-white/5 transition-colors group"
          onclick={() => handleCopy(row.label, row.value)}
          title="Click to copy"
        >
          <span class="text-[var(--text-secondary,#6b7280)] text-[11px] font-medium uppercase tracking-wide shrink-0 pt-0.5">
            {row.label}
          </span>
          <div class="flex items-center gap-1.5 min-w-0">
            {#if copiedLabel === row.label}
              <span class="text-[var(--accent,#0066cc)] text-xs">Copied!</span>
            {:else}
              <span class="text-[var(--text-primary,#f9fafb)] text-xs text-right break-all leading-relaxed">
                {row.value}
              </span>
              <svg
                width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
                class="shrink-0 text-[var(--text-secondary,#9ca3af)] opacity-0 group-hover:opacity-100 transition-opacity"
              >
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
              </svg>
            {/if}
          </div>
        </button>
      {/each}
    {/if}
  </div>
</div>
