<a href="{{ $page->getPath() }}"
   class="p-url"
   itemprop="url"
   @if ($page->description) title="{{ $page->description }}" @endif
   @if ($page->type) type="{{ $page->type }}" @endif
   >{{ $page->link_text ?? $page->title }}</a>
