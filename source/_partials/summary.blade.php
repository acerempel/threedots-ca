<article itemscope itemtype="https://schema.org/BlogPosting" class="h-entry p-space-1/4">
  @if ($post->title)
    <h3 itemprop="headline"
      class="p-name font-size-3 medium sans-serif"
      id="{{ $page->getFilename() }}-title">
      @link(['page' => $post])
    </h3>
  @else
    <p class="colour-lighter light sans-serif">@date($post->date)</p>
  @endif

  @php $excerpt_marker = '<!-- FOLD -->'; @endphp
  @if ($post->synopsis) <p class="h-summary oblique serif" itemprop="abstract">{{ $post->synopsis }}</p>
  @elseif ($post->getContent() !== null && str_contains($post->getContent(), $excerpt_marker))
    <div class="h-summary serif" itemprop="abstract">{!! explode($excerpt_marker, $post->getContent())[0] !!}</div>
    <p class="link-plain serif">
    <a href="{!! $post->getPath() !!}" id="{{ $post->getFilename() }}-read-more"
      aria-labelledby="{{ $post->getFilename() }}-read-more {{ $post->getFilename() }}-title"
      >Continue reading …</a>
    </p>
  @else <div class="e-content prose" itemprop="articleBody">{!! $post->getContent() !!}</div>
  @endif
</article>
