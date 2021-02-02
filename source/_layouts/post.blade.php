@extends('_layouts.base')
@section('body')
    <article class="mx-auto h-entry" id="post" itemscope itemtype="https://schema.org/BlogPosting">
      <header class="sans-serif">
        <p class="colour-lighter light">
          @date($page->date)
          @if ($page->date_revised)
            <small> – Revised @date($page->date_revised)</small>
          @endif
        </p>
        @if ($page->title)
          <h1 class="title p-name font-size-4.5 semibold" itemprop="headline">{{ $page->title }}</h1>
        @endif
      </header>
      <div class="e-content serif" itemprop="articleBody">
        @yield('content')
      </div>
    </article>
@endsection
