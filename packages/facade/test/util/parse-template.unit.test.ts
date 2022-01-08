import { Util } from '../../src/util';

describe('Facade Util.parseTemplate()', () => {
  test('Parses template with {{{...}}} or [[...]]', () => {
    const template = `{{Char info/{{{1|line}}}|size={{{size}}}
|code=4290|main=*qex
|name=Feastful Legamünt
|jpname=めしあがレガムント
|color=Yellow
|color2=Red
|type1=Attack
|type2=Single

|card1=429005
|card2=429006
|card3=429007
|mat1=454206

|nosf=2019/08/29
|acqe=[[PPQ:5th Pwurp Challenge|5th Pwurp Challenge]]
|tokkun=Tmajoratk
}}`;

    const parsed = Util.parseTemplate(template);
    expect(parsed).toEqual<Record<string, string>>({
      size: '{{{size}}}',
      code: '4290',
      main: '*qex',
      name: 'Feastful Legamünt',
      jpname: 'めしあがレガムント',
      color: 'Yellow',
      color2: 'Red',
      type1: 'Attack',
      type2: 'Single',
      card1: '429005',
      card2: '429006',
      card3: '429007',
      mat1: '454206',
      nosf: '2019/08/29',
      acqe: '[[PPQ:5th Pwurp Challenge|5th Pwurp Challenge]]',
      tokkun: 'Tmajoratk',
    });
  });

  test('Parses template that has keys with empty values', () => {
    const template = `{{Card info/{{{1|icon}}}|size={{{size}}}
|code=137406|rarity=6|up=1
|name=Otoshidama Hunter Patri|jpname=おとしだまハンターパトリ

|hp1=|atk1=|rcv1=
|hpmax=|atkmax=|rcvmax=
|hpmax7=|atkmax7=|rcvmax7=

|fint=2022/01/01
|ft=
}}`;
    const parsed = Util.parseTemplate(template);

    expect(parsed).toEqual<Record<string, string>>({
      size: '{{{size}}}',
      code: '137406',
      rarity: '6',
      up: '1',
      name: 'Otoshidama Hunter Patri',
      jpname: 'おとしだまハンターパトリ',
      hp1: '',
      atk1: '',
      rcv1: '',
      hpmax: '',
      atkmax: '',
      rcvmax: '',
      hpmax7: '',
      atkmax7: '',
      rcvmax7: '',
      fint: '2022/01/01',
      ft: '',
    });
  });

  test('Parses template containing brackets with depth >=2', async () => {
    const template = `{{Card info/{{{1|icon}}}|size={{{size}}}
|code=504907|rarity=7|main=Unusual Ecolo
|as=Intimité|jpas=アーンテミッド|asn=40
|ase={{PPQ skilltext/rallyattack|rally=all|front=1|mode=single|atkcolor=colored|colornum=1|rate=7|stat=atk}}, and all cards attack with any color chains for 2 turns (normal attacks only) ({{spectrum|cat=1}}2)
|asp=
{{PPQ asp|asstart=2018/09/21|asend=2020/04/01|color=5
|as=Intimité|jpas=アーンテミッド|asn=40
|ase={{PPQ skilltext/rallyattack|rally=all|front=1|mode=single|atkcolor=colored|colornum=1|rate=3|stat=atk}}, and all cards attack with any color chains for 1 turn (normal attacks only) ({{spectrum|cat=1}}1)
{{PPQ color number up|second=0|base=0|side=1|atk=3}}}}
|asnotes=
* AS Attack power multiplier:
{{PPQ color number up|second=0|base=0|side=1|atk=7}}
|asetym=from ''intimité'' (French: intimacy)
}}`;

    const parsed = Util.parseTemplate(template);
    expect(parsed).toEqual<Record<string, string>>({
      size: '{{{size}}}',
      code: '504907',
      rarity: '7',
      main: 'Unusual Ecolo',
      as: 'Intimité',
      jpas: 'アーンテミッド',
      asn: '40',
      ase: '{{PPQ skilltext/rallyattack|rally=all|front=1|mode=single|atkcolor=colored|colornum=1|rate=7|stat=atk}}, and all cards attack with any color chains for 2 turns (normal attacks only) ({{spectrum|cat=1}}2)',
      asp: '{{PPQ asp|asstart=2018/09/21|asend=2020/04/01|color=5\n|as=Intimité|jpas=アーンテミッド|asn=40\n|ase={{PPQ skilltext/rallyattack|rally=all|front=1|mode=single|atkcolor=colored|colornum=1|rate=3|stat=atk}}, and all cards attack with any color chains for 1 turn (normal attacks only) ({{spectrum|cat=1}}1)\n{{PPQ color number up|second=0|base=0|side=1|atk=3}}}}',
      asnotes:
        '* AS Attack power multiplier:\n{{PPQ color number up|second=0|base=0|side=1|atk=7}}',
      asetym: "from ''intimité'' (French: intimacy)",
    });
  });

  test('Parses template by removing HTML comments', () => {
    const template = `{{Card info/{{{1|icon}}}|size={{{size}}}
|name=Otoshidama Hunter Patri|jpname=おとしだまハンターパトリ
|as=I'll Show You a Landslide Victory...!|jpas=圧勝してみせます…！|aslv=2|asn=<!--40|asref1=traceup
|ase=Transform {{puyo|1}}5 → {{chance|1}}* and adds 6 to trace capacity for 3 turns (same-time coefficient increases to 4×)
|jpase=フィールド上の色ぷよをランダムで5個チャンスぷよに変え、3ターンの間、なぞり消し数を6個増やす（同時消し係数を4倍に）
|asnotes=
* {{PPQ skillnotes/chancepriority|1}}
* AS equivalent increase rates:
{{PPQ stc coeff up|multiple=4}}-->

|ls=Coincident As Calculated|jpls=計算通りの偶然|lslv=2
}}`;

    const parsed = Util.parseTemplate(template);
    expect(parsed).toEqual<Record<string, string>>({
      size: '{{{size}}}',
      name: 'Otoshidama Hunter Patri',
      jpname: 'おとしだまハンターパトリ',
      as: "I'll Show You a Landslide Victory...!",
      jpas: '圧勝してみせます…！',
      aslv: '2',
      asn: '',
      ls: 'Coincident As Calculated',
      jpls: '計算通りの偶然',
      lslv: '2',
    });
  });
});
