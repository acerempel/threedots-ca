@extends('_layouts.base')
@section('body')
<article class="p-space-1/2">
  <header><h1 class="font-size-4.5 bold sans-serif">{{ $page->title }}</h1></header>
  <div class="prose">@yield('content')</div>
</article>
@endsection
