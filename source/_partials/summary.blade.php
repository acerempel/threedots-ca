<article itemscope itemtype="https://schema.org/BlogPosting" class="h-entry post p-space-1/4">
  @if ($post->title)
    <h3 itemprop="headline"
      class="p-name font-size-3 medium"
      id="{{ $page->getFilename() }}-title">
      @link(['page' => $post])
    </h3>
  @else
    <p class="colour-lighter light">@date($post->date)</p>
  @endif

  @php $excerpt_marker = '<!-- FOLD -->'; @endphp
  @if ($post->synopsis) <p class="h-summary oblique" itemprop="abstract">{{ $post->synopsis }}</p>
  @elseif ($post->getContent() !== null && str_contains($post->getContent(), $excerpt_marker))
    <div class="h-summary" itemprop="abstract">{!! explode($excerpt_marker, $post->getContent())[0] !!}</div>
    <p class="link-plain">
    <a href="{!! $post->getPath() !!}" id="{{ $post->getFilename() }}-read-more"
      aria-labelledby="{{ $post->getFilename() }}-read-more {{ $post->getFilename() }}-title"
      >Continue reading …</a>
    </p>
  @else <div class="e-content" itemprop="articleBody">{!! $post->getContent() !!}</div>
  @endif
</article>
