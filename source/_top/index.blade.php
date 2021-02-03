---
title: Good evening!
link_text: Home
description: The website of Alan Rempel, a elliptical human man.
weight: 0
---

@extends('_layouts.base')

@section('body')

<h1 class="font-size-5 bold"><span class="greeting">Good evening</span>!</h1>
<section id="intro" class="p-space-1/2 prose">
  @include('_content.index')
  <aside><small class="colour-lighter light">Last updated on @date(time())</small></aside>
</section>
<section id="blog" class="border-top article-list" itemscope itemtype="https://schema.org/Blog">
  <h2 class="font-size-4 semibold"><a href="/posts">Blog</a></h2>
    @foreach ($posts->take(3) as $post)
    @include('_partials.summary', ['post' => $post])
    @endforeach
  <p class="mt-1"><a href="/posts">See all posts …</a></p>
</section>
<section id="miscellaneous" class="border-top p-space-1/2" >
  <h2 class="font-size-4 semibold">Miscellaneous lists</h2>
  <ul>
    @foreach ($misc->take(3) as $misc_page)
    <li itemscope itemtype="https://schema.org/Article">
      <span itemprop="headline">@link(['page' => $misc_page])</span>
      @if ($misc_page->date_revised)
      <small class="light">— Updated @date($misc_page->date_revised)</small>
      @endif
    </li>
    @endforeach
  </ul>
  <p><a href="/lists">See all lists …</a></p>
</section>

@endsection('body')
