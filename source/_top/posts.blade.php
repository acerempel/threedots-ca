---
title: Blog
description: All posts in the ‘three dots’ blog, grouped by year.
weight: 8
---

@push('DOMContentLoaded')
  const navControl = document.querySelector('.dropdown-nav-control');
  navControl.addEventListener('click', (event) => {
    let dropdown = event.target.closest('.dropdown-nav');
    dropdown.classList.toggle("open");
  });
  navControl.addEventListener('keydown', (event) => {
    if (event.key === "Enter") event.target.closest('.dropdown-nav').classList.toggle('open');
  });
@endpush
@extends('_layouts.base')
@section('body')
      @foreach ($posts->groupBy(function ($item, $key) { return getdate($item->date)['year']; }) as $year => $year_posts)
        @push('years_nav')
          <a class="nav-link" href="#y{{ $year }}">{{ $year }}</a>@unless ($loop->last)<span class="flex-divider"></span>@endunless
        @endpush
        @push('years_list')
        <section id="y{{ $year }}" class="article-list mt-3/4">
          <h2 class="font-size-4 semibold sans-serif swing">{{ $year }}</h2>
            @foreach ($year_posts as $post)
            @include ('_partials.summary', ['post' => $post])
            @endforeach
        </section>
        @endpush
      @endforeach
      <section itemscope itemtype="https://schema.org/Blog">
        <header class="sans-serif flex thingy">
          <h1 class="font-size-5 bold">Blog</h1>
          <nav aria-labelledby="jty" class="dropdown-nav colour-highlight">
            <h2 class="font-size-2 mt-1/4 medium dropdown-nav-control"
                id="jty" tabindex="0"
              >@include ('_partials.caret-right')Jump to year …</h2>
            <span id="years-nav" class="dropdown-nav-items link-plain">@stack('years_nav')</span>
          </nav>
        </header>
        @stack('years_list')
      </section>
@endsection
