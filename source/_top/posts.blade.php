---
title: Blog
description: All posts in the ‘three dots’ blog, grouped by year.
weight: 8
---

@extends('_layouts.base')
@section('body')
      @foreach ($posts->groupBy(function ($item, $key) { return getdate($item->date)['year']; }) as $year => $year_posts)
        @push('years_nav')
          <a class="nav-link" href="#y{{ $year }}">{{ $year }}</a>@unless ($loop->last)<span class="nav-divider">•</span>@endunless
        @endpush
        @push('years_list')
        <section id="y{{ $year }}" class="border-top article-list">
          <h2 class="font-size-4 semibold sans-serif">{{ $year }}</h2>
            @foreach ($year_posts as $post)
            @include ('_partials.summary', ['post' => $post])
            @endforeach
        </section>
        @endpush
      @endforeach
      <section itemscope itemtype="https://schema.org/Blog">
        <header class="sans-serif">
          <h1 class="font-size-5 bold">Blog</h1>
          <nav class="flex row mt-1/4 font-size-2" aria-label="Jump to posts from a specific year" >
            @stack('years_nav')
          </nav>
        </header>
        @stack('years_list')
      </section>
@endsection
