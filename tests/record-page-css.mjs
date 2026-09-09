// Effective CSS over the actual Rust fixtures. This is not a browser layout engine.
import assert from 'node:assert/strict';
import {readFileSync, writeFileSync} from 'node:fs';
import test from 'node:test';
import {parse, el, computed, px} from './css-cascade.mjs';

function markup(path) {
  const nodes = [], stack = [], voids = new Set('meta link input img br hr source wbr'.split(' '));
  for (const [, end, tag, tail] of readFileSync(path, 'utf8').matchAll(/<(\/?)([a-z][\w-]*)\b([^>]*)>/g)) {
    if (end) { while (stack.length && stack.pop().tag !== tag) {} continue; }
    const attrs = Object.fromEntries([...tail.matchAll(/([^\s=]+)(?:="([^"]*)")?/g)].map(m => [m[1],m[2] || '']));
    const node = el(tag, attrs.class || '', attrs, stack.at(-1)); nodes.push(node);
    if (!voids.has(tag)) stack.push(node);
  }
  return {nodes, root:nodes[0], one:name=>{ const node=nodes.find(n=>n.classes.includes(name)); assert(node,name); return node; }};
}
const evidence=[];
for (const file of ['static/maud-ui.css','static/maud-ui.min.css']) {
  const source=readFileSync(file,'utf8'), rules=parse(source), env={width:960,viewportWidth:1280};
  const style=(node,environment=env)=>computed(rules,node,environment);
  test(`${file}: three actual sibling cards get one shared gap, never additive margins`,()=>{
    const fixture=markup('docs/fixtures/record-page-three-cards.html');
    const cards=fixture.nodes.filter(n=>n.classes.includes('mui-card'));
    assert.equal(cards.length,3); assert(cards.every(n=>n.parent===cards[0].parent));
    const stack=style(cards[0].parent);
    assert.equal(stack.display,'flex'); assert.equal(stack['flex-direction'],'column'); assert.equal(px(stack.gap),16);
    for (const card of cards) assert.equal(style(card)['margin-block'],'0');
    for (const width of [390,1280]) assert.equal(px(style(cards[0].parent,{width,viewportWidth:width}).gap),16);
    const broken=parse(source+'\n.mui-stack { gap: 0; }');
    assert.throws(()=>assert.equal(px(computed(broken,cards[0].parent,env).gap),16));
    evidence.push({file,contract:'three sibling cards',siblings:3,gap:16,child_block_margins:0});
  });
  test(`${file}: record identity, action row and following blocks use vertical reading order`,()=>{
    const fixture=markup('docs/fixtures/record-page-comfortable-guest.html');
    for (const name of ['mui-record-page','mui-record-header','mui-record-header__row']) {
      const css=style(fixture.one(name)); assert.equal(css['flex-direction'],'column'); assert.equal(px(css.gap),16);
    }
    assert.equal(style(fixture.one('mui-record-header'))['padding-top'],'0');
    assert.equal(style(fixture.one('mui-record-header__identity')).flex,'none');
    const subtitle=style(fixture.one('mui-record-header__subtitle'));
    assert.equal(subtitle.color,subtitle.token('--mui-text-muted')); assert.equal(subtitle['white-space'],'nowrap');
  });
  test(`${file}: fact sections respond to their container and a singleton remains one row`,()=>{
    const fixture=markup('docs/fixtures/record-page-comfortable-guest.html');
    const grid=fixture.one('mui-record-facts__grid');
    for (const [width,columns] of [[240,1],[480,2],[720,3],[960,4]]) {
      const css=style(grid,{width,viewportWidth:1280});
      assert.equal(css['grid-template-columns'].replaceAll(' ',''), columns===1?'minmax(0,1fr)':`repeat(${columns},minmax(0,1fr))`);
    }
    const root=fixture.one('mui-record-facts'); root.attrs['data-columns']='2';
    assert.equal(style(grid)['grid-template-columns'].replaceAll(' ',''),'repeat(2,minmax(0,1fr))');
    const single=fixture.nodes.find(n=>n.attrs['data-single']==='true');
    const singleGrid=fixture.nodes.find(n=>n.parent===single && n.tag==='dl');
    assert.equal(style(singleGrid)['grid-template-columns'].replaceAll(' ',''),'minmax(0,1fr)');
    const minimal=markup('docs/fixtures/record-page-minimal.html'), inline=style(minimal.one('mui-record-facts'));
    assert.equal(inline.border,'0'); assert.equal(inline.background,'transparent'); assert.equal(inline['padding-top'],'0');
  });
  test(`${file}: masked values use mono while labels and hints remain muted`,()=>{
    const fixture=markup('docs/fixtures/record-page-comfortable-guest.html');
    const hint=fixture.one('mui-record-facts__masked'), value=style(hint.parent), label=style(fixture.one('mui-record-facts__label'));
    assert.equal(value['font-family'],value.token('--mui-font-mono'));
    assert.equal(label.color,label.token('--mui-text-muted')); assert.equal(style(hint).color,label.color);
    assert.equal(px(style(fixture.one('mui-record-facts'))['border-radius']),12);
  });
  test(`${file}: related records keep compact link identity, status and date, including touch targets`,()=>{
    const fixture=markup('docs/fixtures/record-page-comfortable-guest.html');
    const identity=fixture.one('mui-related-list__identity'), css=style(identity);
    assert.equal(css.color,css.token('--mui-link'));
    assert.equal(css['min-width'],'0');
    assert.equal(style(fixture.one('mui-related-list__item')).display,'flex');
    assert.equal(style(fixture.one('mui-related-list__title'))['text-overflow'],'ellipsis');
    assert.equal(px(style(identity,{width:390,viewportWidth:390,coarse:true})['min-height']),44);
    assert.equal(style(fixture.one('mui-related-list__empty')).color,css.token('--mui-text-muted'));
  });
  test(`${file}: every density reserves the shortcut badge inside its own input field`,()=>{
    for (const density of ['compact','comfortable','spacious']) {
      const fixture=markup(`docs/fixtures/record-page-${density}-guest.html`);
      const field=fixture.one('mui-page-header__search-field'), input=fixture.nodes.find(n=>n.parent===field && n.tag==='input'), badge=fixture.nodes.find(n=>n.parent===field && n.tag==='kbd');
      assert(input && badge); assert.equal(input.attrs.placeholder,'Search');
      assert.equal(style(field).position,'relative');
      const inputCss=style(input), badgeCss=style(badge);
      const reserved=px(inputCss['padding-inline-end']), width=px(badgeCss.width), inset=px(badgeCss['inset-inline-end']);
      assert.equal(reserved,52); assert(reserved>=width+2*inset);
      assert.equal(style(badge,{width:390,viewportWidth:390}).display,'none');
      assert.equal(px(style(input,{width:390,viewportWidth:390})['padding-inline-end']),12);
      const broken=parse(source+'\n.mui-page-header__search-panel .mui-input { padding-inline-end: 12px; }');
      assert.throws(()=>assert(px(computed(broken,input,env)['padding-inline-end'])>=width+2*inset));
      evidence.push({file,contract:'search clearance',density,input_inline_end:reserved,badge_width:width,badge_inset:inset});
    }
  });
  test(`${file}: breadcrumb context shrinks; middle ellipsizes and the current crumb wraps`,()=>{
    const fixture=markup('docs/fixtures/record-page-comfortable-long.html');
    const context=style(fixture.one('mui-page-header__context'));
    assert.equal(context['min-width'],'0'); assert.equal(context.overflow,'hidden');
    const current=fixture.nodes.find(n=>n.attrs['data-position']==='current' && n.classes.includes('mui-breadcrumb__item'));
    const middle=fixture.nodes.find(n=>n.attrs['data-position']==='middle');
    const middleLink=fixture.nodes.find(n=>n.parent===middle && n.tag==='a');
    assert.equal(style(middleLink)['text-overflow'],'ellipsis');
    assert.equal(style(current)['white-space'],'normal'); assert.equal(style(current)['min-width'],'0');
    assert.equal(fixture.nodes.filter(n=>n.classes.includes('mui-breadcrumb__separator')).length,2);
    const broken=parse(source+'\n.mui-page-header__context .mui-breadcrumb__item[data-position="current"] { min-width: max-content; }');
    assert.throws(()=>assert.equal(computed(broken,current,env)['min-width'],'0'));
  });
}
test.after(()=>writeFileSync('docs/night-5-record-contract.json',JSON.stringify({method:'CSS cascade over Rust fixture markup; not browser geometry',cases:evidence},null,2)+'\n'));
