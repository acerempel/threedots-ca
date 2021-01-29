@extends('_layouts.base')
@section('body')
<article>
  <header><h1 class="font-size-4.5 bold">{{ $page->title }}</h1></header>
  @yield('content')
</article>
@endsection
