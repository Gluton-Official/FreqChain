---

excalidraw-plugin: parsed
tags: [excalidraw]

---
==⚠  Switch to EXCALIDRAW VIEW in the MORE OPTIONS menu of this document. ⚠==


# Text Elements
FreqChain ^Rhmfb9Eg

-params: Arc<FreqChainParams>
-sample_rate: Arc<AtomicF32>
-equalizer: Equalizer<7, 2>
-frequency_sidechain: FrequencySidechain ^5oOvbdno

+params(&self) -> Arc<dyn Params>
+editor(&mut self,...) -> Option<Box<dyn Editor>>
+initialize(&mut self,...) -> bool
+reset(&mut self)
+process(&mut self,...) -> ProcessStatus
+default() -> Self ^4xOO46UC

FreqChainParams ^GyC1qSfA

+editor_state: Arc<IcedState>
+mono_processing: BoolParam
+sidechain_input: SidechainInputParams
+equalizer: EqualizerParams<7>
+frequency_sidechain: FrequencySidechainParams ^gHvFqpTt

+default() -> Self ^Yadmd5AC

#[derive(Params)] ^JEW1fwo7

freqchain ^2sMWkiDY

SidechainInputParams ^aSLxhap1

+solo: BoolParam
+gain: FloatParam ^3N7w9Uoc

+default() -> Self ^9dakj58u

#[derive(Params)] ^ht67OLPq

<<crate>>
nih_plug ^LX3CkqyX

ClapPlugin ^8AAFi6jI

<<trait>> ^RySdWnnp

equalizer ^2S17Bx84

Equalizer<const BANDS: usize, const CHANNELS: usize> ^la8gm2aK

-sample_rate: Option<f32>
-biquad_filters: [BiquadFilter; BANDS]
-x1, x2, y1, y2: [f32: CHANNELS] ^tizupAbT

+process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<BANDS>)
+set_sample_rate(&mut self, sample_rate: f32)
+default() -> Self ^p0fNJLeP

EqualizerParams ^9DUsiLiT

+bands: [BandParams; BANDS]
+bypass: BoolParam ^P84bhVxS

+default() -> Self ^eoXonYce

#[derive(Params)] ^R6alQ75K

<const BANDS: usize> ^o4bTWuvb

BandParams ^Yso3XsNd

+band_type: EnumParam<BandType>
+frequency: FloatParam
+q: FloatParam
+gain: FloatParam
-dirty: Arc<AtomicBool> ^9CwkSl57

-new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self
-calculate_filter(&self, sample_rate: f32) -> BiquadFilter ^tM036NAn

#[derive(Params)] ^4kC3drPb

BandType ^azlC8eTN

+Peak
+Notch
+HighShelf
+LowShelf
+HighPass
+LowPass ^tcox6TkE

<<enum>> ^FbJV8KoC

#[derive(Enum, Debug, PartialEq)] ^dJ2pGwAg

frequency_sidechain ^GUPO0U47

FrequencySidechain ^vulOTFL3

-stft: StftHelper<1>
-channels: usize
-window_size: usize
-overlap_times: usize
-gain_compensation: f32
-forward_fft: ForwardFFT<f32>
-inverse_fft: InverseFFT<f32>
-main_complex_buffer: Vec<Complex32>
-sidechain_complex_buffer: Vec<Vec<Complex32>>
-window_function: Vec<f32>
-smoother: Vec<Vec<Smoother>> ^fP7ZoJJI

+new(channels: usize, window_size: usize, hop_size: usize) -> Self
+process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer,
    sample_rate: f32, params: &FrequencySidechainParams)
+latency_samples(&self) -> u32
+reset(&mut self) ^LlntZxYQ

FrequencySidechainParams ^7otgJBOs

+detail: FloatParam
+precision: FloatParam
+smoother: SmootherParams ^AXGlc8Sy

+default() -> Self ^lPGcrI1j

#[derive(Params)] ^7sM2dvIX

smoother ^3dzTBa6a

SmootherParams ^zojWDKOv

+attack_bypass: BoolParam
+attack_speed: FloatParam
+decay_bypass: BoolParam
+decay_speed: FloatParam ^YdCW6qFn

+default() -> Self ^q2cHqNvg

#[derive(Params)] ^f66qOwQW

Smoother ^3DEUtDeP

-current_value: Option<f32> ^BCfEhxIF

+process(&mut self, value: &mut f32, sample_rate: f32, params: &SmootherParams) ^S83fr9eX

#[derive(Default, Clone)] ^kQhqgKQe

modules ^3qN9D1TV

util ^xAgytTW9

BiquadFilter ^IfgzOHqG

+b0, b1, b2, a1, a2: f32 ^EDShATxL

+biquad_transform(&self, sample: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> f32 ^ULcj07L1

#[derive(Debug, Default, Copy, Clone)] ^XVeGAe9G

BufferUtils ^HyoCW4im

+on_each_sample<F>(&mut self, f: F) where F: FnMut(usize, usize, &mut f32) ^TkD3AmMN

<<trait>> ^ugkmWwXs

fft ^orq9JwKH

FFT<T: FftNum> ^ytFH8Su9

+new(planner: &mut RealFftPlanner<T>, length: usize) -> Self
+get_length(&self) -> usize
+create_real_buffer(&self) -> Vec<T>
+create_complex_buffer(&self) -> Vec<Complex<T>> ^I8JCKZgD

<<trait>> ^ybd6PJ1O

ForwardFFT<T: FftNum> ^wSkR0QkP

-plan: Arc<dyn RealToComplex<T>>
-length: usize ^QRuWwLus

+process(&self, input: &mut [T], output: &mut [Complex<T>]) ^El9o25aL

InverseFFT<T: FftNum> ^GfpZB745

-plan: Arc<dyn ComplexToReal<T>>
-length: usize ^UJAq9NYu

+process(&self, input: &mut [Complex<T>], output: &mut [T]) ^y7MAjAEm

ui ^2V00ZVIP

editor ^9ytaNbkv

FreqChainEditor ^SgjLIWYP

-params: Arc<FreqChainParams>
-context: Arc<dyn GuiContext>
-sample_rate: Arc<AtomicF32> ^2smsFtVk

Message ^mVpRoBZS

+ParamUpdate(ParamMessage) ^VDYEU7YO

<<enum>> ^BL3ZbE7B

#[derive(Debug, Clone, Copy)] ^UmD7E25g

Plugin ^vdKWz8pN

<<trait>> ^FJ3dc3ia

Vst3Plugin ^FaTSVJwb

<<trait>> ^d1Ihn8cW

IcedEditor ^voD3ySNH

<<trait>> ^RFkGQ9nf

Params ^B4EChb8q

<<trait>> ^INxBGIo7

%%
# Drawing
```json
{
	"type": "excalidraw",
	"version": 2,
	"source": "https://excalidraw.com",
	"elements": [
		{
			"type": "arrow",
			"version": 73,
			"versionNonce": 732281549,
			"isDeleted": false,
			"id": "3TIfSn3w",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1769.207436668045,
			"y": 1197.5006237368204,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 275.5758927787633,
			"height": 825.8823596414063,
			"seed": 80231,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-270.1177767717329,
					0
				],
				[
					-267.66510701307834,
					-296.7573145419651
				],
				[
					-268.6335550262395,
					-297.2535769439802
				],
				[
					-269.38079097733134,
					-297.3647273026021
				],
				[
					-270.1135392623373,
					-297.54856396827756
				],
				[
					-270.82474711417467,
					-297.80331749683205
				],
				[
					-271.5075690943354,
					-298.1265358630104
				],
				[
					-272.1554329808391,
					-298.5151080614371
				],
				[
					-272.7621030264497,
					-298.9652940503182
				],
				[
					-273.3217399782948,
					-299.4727607496744
				],
				[
					-273.8289572811914,
					-300.0326237476197
				],
				[
					-274.27887292371634,
					-300.6394943132573
				],
				[
					-274.6671564279982,
					-301.2875312636918
				],
				[
					-274.9900705309494,
					-301.9704971859318
				],
				[
					-275.24450715575495,
					-302.68181847254107
				],
				[
					-275.42801732738644,
					-303.4146485931923
				],
				[
					-275.53883474420354,
					-304.16193399312715
				],
				[
					-275.5758927787633,
					-304.91648198424537
				],
				[
					-275.53883474420354,
					-305.6710299753636
				],
				[
					-275.42801732738644,
					-306.41831537529845
				],
				[
					-275.24450715575495,
					-307.15114549594966
				],
				[
					-274.9900705309494,
					-307.86246678255895
				],
				[
					-274.6671564279982,
					-308.54543270479894
				],
				[
					-274.27887292371634,
					-309.19346965523357
				],
				[
					-273.8289572811914,
					-309.800340220871
				],
				[
					-273.3217399782948,
					-310.3602032188163
				],
				[
					-272.7621030264497,
					-310.8676699181725
				],
				[
					-272.1554329808391,
					-311.3178559070536
				],
				[
					-271.5075690943354,
					-311.7064281054803
				],
				[
					-270.82474711417467,
					-312.0296464716587
				],
				[
					-270.1135392623373,
					-312.28440000021317
				],
				[
					-269.38079097733134,
					-312.46823666588864
				],
				[
					-268.6335550262395,
					-312.57938702451054
				],
				[
					-267.8790236212128,
					-312.6167812440417
				],
				[
					-267.6039950091931,
					-825.8823596414063
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1587,
			"versionNonce": 381805699,
			"isDeleted": false,
			"id": "JCtGuFUWol-LQHJgropM4",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2544.264207993731,
			"y": -875.8936092940532,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 863,
			"height": 1174,
			"seed": 330556909,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "xAgytTW9",
					"type": "text"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1284,
			"versionNonce": 438229293,
			"isDeleted": false,
			"id": "xAgytTW9",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2941.764207993731,
			"y": -870.8936092940532,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 68,
			"height": 34,
			"seed": 509049699,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "util",
			"rawText": "util",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "JCtGuFUWol-LQHJgropM4",
			"originalText": "util"
		},
		{
			"type": "rectangle",
			"version": 883,
			"versionNonce": 1052269603,
			"isDeleted": false,
			"id": "9lhHwdDcsRrspBmRNOsjW",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2619.613712582667,
			"y": -646.0591201034358,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 725,
			"height": 690,
			"seed": 813718563,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "orq9JwKH",
					"type": "text"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 606,
			"versionNonce": 1822763917,
			"isDeleted": false,
			"id": "orq9JwKH",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2963.613712582667,
			"y": -641.0591201034358,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 37,
			"height": 24,
			"seed": 1532832653,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "fft",
			"rawText": "fft",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "9lhHwdDcsRrspBmRNOsjW",
			"originalText": "fft"
		},
		{
			"type": "rectangle",
			"version": 1001,
			"versionNonce": 2084495299,
			"isDeleted": false,
			"id": "EPUmuZB0uEYAlK0xJpUHy",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 294.3525383543347,
			"y": 423.96138793451246,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 3187,
			"height": 1210,
			"seed": 175239213,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "3qN9D1TV",
					"type": "text"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 748,
			"versionNonce": 326250989,
			"isDeleted": false,
			"id": "3qN9D1TV",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1829.3525383543347,
			"y": 428.96138793451246,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 117,
			"height": 34,
			"seed": 33961251,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "modules",
			"rawText": "modules",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "EPUmuZB0uEYAlK0xJpUHy",
			"originalText": "modules"
		},
		{
			"type": "rectangle",
			"version": 687,
			"versionNonce": 963899235,
			"isDeleted": false,
			"id": "ZQOpGQnL_t-ZpOg8cLJGQ",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1603.5216131034754,
			"y": -687.098165073304,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 607,
			"height": 964,
			"seed": 1934498403,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"type": "text",
					"id": "2sMWkiDY"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": true
		},
		{
			"type": "rectangle",
			"version": 1050,
			"versionNonce": 999255117,
			"isDeleted": false,
			"id": "cMVtsGJjEr-NfgicedjH6",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1659.4886199575797,
			"y": -631.7787477978235,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 481.0006726026877,
			"height": 284.80115960790386,
			"seed": 447035779,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 774,
			"versionNonce": 413534979,
			"isDeleted": false,
			"id": "k8f3Il4OjTvCv8vUToCt4",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1659.2208574689673,
			"y": -577.9192419124352,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 481.63487364426544,
			"height": 0,
			"seed": 1969819939,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					240.81743682213272,
					0
				],
				[
					481.63487364426544,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1212,
			"versionNonce": 886463149,
			"isDeleted": false,
			"id": "LtvWetjabWIY4nopSJYNG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1826.8478359006924,
			"y": -628.234441251961,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 44,
			"seed": 356065645,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"type": "text",
					"id": "Rhmfb9Eg"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1148,
			"versionNonce": 1850978979,
			"isDeleted": false,
			"id": "Rhmfb9Eg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1832.3478359006924,
			"y": -623.234441251961,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 150,
			"height": 34,
			"seed": 884479021,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476626,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FreqChain",
			"rawText": "FreqChain",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "LtvWetjabWIY4nopSJYNG",
			"originalText": "FreqChain"
		},
		{
			"type": "rectangle",
			"version": 1564,
			"versionNonce": 401636621,
			"isDeleted": false,
			"id": "XgHztKyg8C9bBHqREncQT",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1666.069827241463,
			"y": -575.2140607529903,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 467,
			"height": 86,
			"seed": 1694892995,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"type": "text",
					"id": "5oOvbdno"
				},
				{
					"id": "38JRRy_gDquzUOviyqGyY",
					"type": "arrow"
				}
			],
			"updated": 1734013476626,
			"link": null,
			"locked": false
		},
		{
			"type": "line",
			"version": 880,
			"versionNonce": 2066441795,
			"isDeleted": false,
			"id": "rxpZ4oXqUsqDx85XItnM1",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1660.585386614579,
			"y": -482.0495115805883,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 480.69229552573984,
			"height": 0,
			"seed": 1617651341,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					240.34614776286992,
					0
				],
				[
					480.69229552573984,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1464,
			"versionNonce": 743920493,
			"isDeleted": false,
			"id": "BcP6yjreWE2WQWLXqZbiw",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1662.535242407761,
			"y": -476.36426135076886,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 470,
			"height": 124,
			"seed": 1570163555,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"type": "text",
					"id": "4xOO46UC"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1552,
			"versionNonce": 1712039395,
			"isDeleted": false,
			"id": "5oOvbdno",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1671.069827241463,
			"y": -570.2140607529903,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 377,
			"height": 76,
			"seed": 1931608835,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-params: Arc<FreqChainParams>\n-sample_rate: Arc<AtomicF32>\n-equalizer: Equalizer<7, 2>\n-frequency_sidechain: FrequencySidechain",
			"rawText": "-params: Arc<FreqChainParams>\n-sample_rate: Arc<AtomicF32>\n-equalizer: Equalizer<7, 2>\n-frequency_sidechain: FrequencySidechain",
			"baseline": 72,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "XgHztKyg8C9bBHqREncQT",
			"originalText": "-params: Arc<FreqChainParams>\n-sample_rate: Arc<AtomicF32>\n-equalizer: Equalizer<7, 2>\n-frequency_sidechain: FrequencySidechain"
		},
		{
			"type": "text",
			"version": 1131,
			"versionNonce": 240410061,
			"isDeleted": false,
			"id": "4xOO46UC",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1667.535242407761,
			"y": -471.36426135076886,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 460,
			"height": 114,
			"seed": 2054925923,
			"groupIds": [
				"n_EZp30Cy6f59kU0RcoGD"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+params(&self) -> Arc<dyn Params>\n+editor(&mut self,...) -> Option<Box<dyn Editor>>\n+initialize(&mut self,...) -> bool\n+reset(&mut self)\n+process(&mut self,...) -> ProcessStatus\n+default() -> Self",
			"rawText": "+params(&self) -> Arc<dyn Params>\n+editor(&mut self,...) -> Option<Box<dyn Editor>>\n+initialize(&mut self,...) -> bool\n+reset(&mut self)\n+process(&mut self,...) -> ProcessStatus\n+default() -> Self",
			"baseline": 110,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "BcP6yjreWE2WQWLXqZbiw",
			"originalText": "+params(&self) -> Arc<dyn Params>\n+editor(&mut self,...) -> Option<Box<dyn Editor>>\n+initialize(&mut self,...) -> bool\n+reset(&mut self)\n+process(&mut self,...) -> ProcessStatus\n+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 1022,
			"versionNonce": 1979904387,
			"isDeleted": false,
			"id": "e7O26kkUsG4o-6AZPyK6V",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1676.4030547501868,
			"y": -272.25536936358174,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 452.72332904692166,
			"height": 231.60648599700008,
			"seed": 1552799149,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 815,
			"versionNonce": 352394285,
			"isDeleted": false,
			"id": "hLUmTkiGHlCqR53lHO1dm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1678.2111761043855,
			"y": -196.85382087088146,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 450.5297957329228,
			"height": 0,
			"seed": 1545955235,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					225.2648978664614,
					0
				],
				[
					450.5297957329228,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1150,
			"versionNonce": 206555427,
			"isDeleted": false,
			"id": "PtknqqHm8LuQr177Do9Tn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1683.1245653878714,
			"y": -245.6781641124685,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 437,
			"height": 44,
			"seed": 1895864333,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "GyC1qSfA",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1107,
			"versionNonce": 2017922701,
			"isDeleted": false,
			"id": "GyC1qSfA",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1777.6245653878714,
			"y": -240.6781641124685,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 248,
			"height": 34,
			"seed": 251558723,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FreqChainParams",
			"rawText": "FreqChainParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "PtknqqHm8LuQr177Do9Tn",
			"originalText": "FreqChainParams"
		},
		{
			"type": "rectangle",
			"version": 1487,
			"versionNonce": 1583393987,
			"isDeleted": false,
			"id": "diALGPAUoIIwtERoUNBZo",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1681.9842620340696,
			"y": -195.14566661542892,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 444,
			"height": 107,
			"seed": 1118572141,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "gHvFqpTt",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 823,
			"versionNonce": 920630509,
			"isDeleted": false,
			"id": "LiiWaGXawxtHzJ60QYujA",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1677.4998214071863,
			"y": -81.9317835445961,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 450.5297957329228,
			"height": 0,
			"seed": 414456547,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					225.2648978664614,
					0
				],
				[
					450.5297957329228,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1368,
			"versionNonce": 1876171875,
			"isDeleted": false,
			"id": "cQRehmP2O1CLPMODfDtaX",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1682.6574189697249,
			"y": -76.21959854107399,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 438,
			"height": 30,
			"seed": 274315469,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "Yadmd5AC",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1235,
			"versionNonce": 1012174669,
			"isDeleted": false,
			"id": "gHvFqpTt",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1686.9842620340696,
			"y": -190.14566661542892,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 433,
			"height": 95,
			"seed": 97830531,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+editor_state: Arc<IcedState>\n+mono_processing: BoolParam\n+sidechain_input: SidechainInputParams\n+equalizer: EqualizerParams<7>\n+frequency_sidechain: FrequencySidechainParams",
			"rawText": "+editor_state: Arc<IcedState>\n+mono_processing: BoolParam\n+sidechain_input: SidechainInputParams\n+equalizer: EqualizerParams<7>\n+frequency_sidechain: FrequencySidechainParams",
			"baseline": 91,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "diALGPAUoIIwtERoUNBZo",
			"originalText": "+editor_state: Arc<IcedState>\n+mono_processing: BoolParam\n+sidechain_input: SidechainInputParams\n+equalizer: EqualizerParams<7>\n+frequency_sidechain: FrequencySidechainParams"
		},
		{
			"type": "text",
			"version": 1078,
			"versionNonce": 1282971651,
			"isDeleted": false,
			"id": "Yadmd5AC",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1687.6574189697249,
			"y": -71.21959854107399,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 171,
			"height": 19,
			"seed": 61019949,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+default() -> Self",
			"rawText": "+default() -> Self",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "cQRehmP2O1CLPMODfDtaX",
			"originalText": "+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 2096,
			"versionNonce": 1191124397,
			"isDeleted": false,
			"id": "LasR3Y-hchWEsAL4jdU2J",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1682.514862935176,
			"y": -267.5878769342012,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 440,
			"height": 31,
			"seed": 1054721187,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "JEW1fwo7",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2084,
			"versionNonce": 38576035,
			"isDeleted": false,
			"id": "JEW1fwo7",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1822.014862935176,
			"y": -261.5878769342012,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 2051574541,
			"groupIds": [
				"s4uI_v6by1To8Ug7xW9li"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "LasR3Y-hchWEsAL4jdU2J",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "text",
			"version": 460,
			"versionNonce": 235462669,
			"isDeleted": false,
			"id": "2sMWkiDY",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1853.5216131034754,
			"y": -682.098165073304,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 107,
			"height": 24,
			"seed": 1429512045,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "freqchain",
			"rawText": "freqchain",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "ZQOpGQnL_t-ZpOg8cLJGQ",
			"originalText": "freqchain"
		},
		{
			"type": "rectangle",
			"version": 1126,
			"versionNonce": 1535030083,
			"isDeleted": false,
			"id": "J8HF28WK_DhZZGqrGGAVz",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1730.281145464294,
			"y": 43.28310279546065,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 355.6377828387916,
			"height": 170.0079213574799,
			"seed": 119032643,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 999,
			"versionNonce": 661345901,
			"isDeleted": false,
			"id": "F2DGEs9JuaL8prWHLySvk",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1732.0892668184927,
			"y": 118.68465128816092,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 352.5016714062672,
			"height": 0,
			"seed": 1305951853,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					176.2508357031336,
					0
				],
				[
					352.5016714062672,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1314,
			"versionNonce": 1483559651,
			"isDeleted": false,
			"id": "BePEBH5GEBFMcPkonbHom",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1737.945234220504,
			"y": 69.86030804657389,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 342,
			"height": 44,
			"seed": 219409123,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "aSLxhap1",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1244,
			"versionNonce": 770703565,
			"isDeleted": false,
			"id": "aSLxhap1",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1743.945234220504,
			"y": 74.86030804657389,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 330,
			"height": 34,
			"seed": 1855908045,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "SidechainInputParams",
			"rawText": "SidechainInputParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "BePEBH5GEBFMcPkonbHom",
			"originalText": "SidechainInputParams"
		},
		{
			"type": "rectangle",
			"version": 1594,
			"versionNonce": 1772551811,
			"isDeleted": false,
			"id": "HXYsKwef-bYnFWma2JpSV",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1735.8623527481768,
			"y": 120.39280554361358,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 339,
			"height": 48,
			"seed": 2109380227,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "3N7w9Uoc",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 956,
			"versionNonce": 657481517,
			"isDeleted": false,
			"id": "KFExG374a2dXxhDh-XrGJ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1731.3779121212935,
			"y": 174.60668861444628,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 354.38682764331827,
			"height": 0,
			"seed": 1103251245,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					177.19341382165916,
					0
				],
				[
					354.3868276433183,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1476,
			"versionNonce": 1776956963,
			"isDeleted": false,
			"id": "kvVbYn7E3a4rv6D5E-Wyd",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1736.535509683832,
			"y": 179.3188736179684,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 341,
			"height": 29,
			"seed": 1741344291,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "9dakj58u",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1332,
			"versionNonce": 697692557,
			"isDeleted": false,
			"id": "3N7w9Uoc",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1740.8623527481768,
			"y": 125.39280554361358,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 38,
			"seed": 1772804493,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+solo: BoolParam\n+gain: FloatParam",
			"rawText": "+solo: BoolParam\n+gain: FloatParam",
			"baseline": 34,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "HXYsKwef-bYnFWma2JpSV",
			"originalText": "+solo: BoolParam\n+gain: FloatParam"
		},
		{
			"type": "text",
			"version": 1150,
			"versionNonce": 1478851011,
			"isDeleted": false,
			"id": "9dakj58u",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1741.535509683832,
			"y": 184.3188736179684,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 171,
			"height": 19,
			"seed": 2117276099,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+default() -> Self",
			"rawText": "+default() -> Self",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "kvVbYn7E3a4rv6D5E-Wyd",
			"originalText": "+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 2174,
			"versionNonce": 1113208813,
			"isDeleted": false,
			"id": "OmAH2T0I0P5V3n-RB3U3h",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1736.3929536492833,
			"y": 47.95059522484121,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 342,
			"height": 29,
			"seed": 2140282861,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "ht67OLPq",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2162,
			"versionNonce": 1366772067,
			"isDeleted": false,
			"id": "ht67OLPq",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1826.8929536492833,
			"y": 52.95059522484121,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 303161699,
			"groupIds": [
				"n6UlhceYSgWvzLMIKXt80"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "OmAH2T0I0P5V3n-RB3U3h",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "rectangle",
			"version": 1331,
			"versionNonce": 461060589,
			"isDeleted": false,
			"id": "16fGKywqmZAH4JVbvD9vl",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 170.8950839731474,
			"y": -792.7290463313174,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 356,
			"height": 829,
			"seed": 605337421,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "LX3CkqyX",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1017,
			"versionNonce": 1482647395,
			"isDeleted": false,
			"id": "LX3CkqyX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 295.8950839731474,
			"y": -787.7290463313174,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 106,
			"height": 48,
			"seed": 1580134403,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "<<crate>>\nnih_plug",
			"rawText": "<<crate>>\nnih_plug",
			"baseline": 43,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "16fGKywqmZAH4JVbvD9vl",
			"originalText": "<<crate>>\nnih_plug"
		},
		{
			"type": "rectangle",
			"version": 1626,
			"versionNonce": 2119996621,
			"isDeleted": false,
			"id": "-EfbZKtkW9ERdkquw4z_A",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 234.29577112105335,
			"y": -570.1291279928375,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 229.6488978624767,
			"height": 73.50945273923699,
			"seed": 726336963,
			"groupIds": [
				"MLyWHNRAnkpQTy5Qqlwxf"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "38JRRy_gDquzUOviyqGyY",
					"type": "arrow"
				}
			],
			"updated": 1734013514028,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1799,
			"versionNonce": 996311811,
			"isDeleted": false,
			"id": "KUEUj9LBobsDYVwq8gAve",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 241.95985987726317,
			"y": -543.5519227417242,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 215,
			"height": 44,
			"seed": 1074484579,
			"groupIds": [
				"MLyWHNRAnkpQTy5Qqlwxf"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "8AAFi6jI",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1737,
			"versionNonce": 71131821,
			"isDeleted": false,
			"id": "8AAFi6jI",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 266.45985987726317,
			"y": -538.5519227417242,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 166,
			"height": 34,
			"seed": 420046413,
			"groupIds": [
				"MLyWHNRAnkpQTy5Qqlwxf"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "ClapPlugin",
			"rawText": "ClapPlugin",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "KUEUj9LBobsDYVwq8gAve",
			"originalText": "ClapPlugin"
		},
		{
			"type": "rectangle",
			"version": 2679,
			"versionNonce": 545512589,
			"isDeleted": false,
			"id": "GlcesuWN_Q3DCMuusGC_4",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 240.4075793060424,
			"y": -565.4616355634569,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 219,
			"height": 29,
			"seed": 1755024749,
			"groupIds": [
				"MLyWHNRAnkpQTy5Qqlwxf"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "RySdWnnp",
					"type": "text"
				}
			],
			"updated": 1734013510705,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 2670,
			"versionNonce": 60578669,
			"isDeleted": false,
			"id": "RySdWnnp",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 306.9075793060424,
			"y": -560.4616355634569,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 717229027,
			"groupIds": [
				"MLyWHNRAnkpQTy5Qqlwxf"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "GlcesuWN_Q3DCMuusGC_4",
			"originalText": "<<trait>>"
		},
		{
			"type": "rectangle",
			"version": 699,
			"versionNonce": 1016201187,
			"isDeleted": false,
			"id": "E6-yD0VbDMDNgpA2MDUw0",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 383.6095243285956,
			"y": 550.8886699267118,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 996,
			"height": 1038,
			"seed": 1512077443,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "2S17Bx84",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 471,
			"versionNonce": 226010061,
			"isDeleted": false,
			"id": "2S17Bx84",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 828.1095243285956,
			"y": 555.8886699267118,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 107,
			"height": 24,
			"seed": 816022829,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "equalizer",
			"rawText": "equalizer",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "E6-yD0VbDMDNgpA2MDUw0",
			"originalText": "equalizer"
		},
		{
			"type": "rectangle",
			"version": 1230,
			"versionNonce": 514722691,
			"isDeleted": false,
			"id": "-P4KOR0T4IpYHMa7TZw7n",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 428.64021040336536,
			"y": 664.1298653775959,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 883.5993887744917,
			"height": 214.52622524492384,
			"seed": 571855373,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 917,
			"versionNonce": 1940693549,
			"isDeleted": false,
			"id": "6czPq68lZb0JxVegpENAK",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 430.32297932809536,
			"y": 719.2174520487468,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 879.8435541337126,
			"height": 0,
			"seed": 275898691,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					439.92177706685635,
					0
				],
				[
					879.8435541337127,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 2043,
			"versionNonce": 1686610723,
			"isDeleted": false,
			"id": "RHqjUawu7REukA4xjrIe_",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 431.3601388709751,
			"y": 669.3170111351778,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 875,
			"height": 44,
			"seed": 1359612013,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "la8gm2aK",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2015,
			"versionNonce": 1133630605,
			"isDeleted": false,
			"id": "la8gm2aK",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 441.3601388709751,
			"y": 674.3170111351778,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 855,
			"height": 34,
			"seed": 583712995,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Equalizer<const BANDS: usize, const CHANNELS: usize>",
			"rawText": "Equalizer<const BANDS: usize, const CHANNELS: usize>",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "RHqjUawu7REukA4xjrIe_",
			"originalText": "Equalizer<const BANDS: usize, const CHANNELS: usize>"
		},
		{
			"type": "rectangle",
			"version": 1567,
			"versionNonce": 1452099267,
			"isDeleted": false,
			"id": "INAa1si8v5qCQT8OJCBjm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 431.480419970982,
			"y": 723.6269945818058,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 467,
			"height": 67,
			"seed": 1424519885,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tizupAbT",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1030,
			"versionNonce": 417446637,
			"isDeleted": false,
			"id": "_xkS0v6fD2Xw8ieeNaguu",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 429.7369770603648,
			"y": 799.1967478207551,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 880.9606350040712,
			"height": 0,
			"seed": 572959875,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					440.4803175020356,
					0
				],
				[
					880.9606350040712,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1653,
			"versionNonce": 727545443,
			"isDeleted": false,
			"id": "LDbw2efYTF0EN5eR2hzEe",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 433.63736426688865,
			"y": 804.4943554417785,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 706,
			"height": 67,
			"seed": 2051707181,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "p0fNJLeP",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1683,
			"versionNonce": 1545983309,
			"isDeleted": false,
			"id": "tizupAbT",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 436.480419970982,
			"y": 728.6269945818058,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 357,
			"height": 57,
			"seed": 306455587,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-sample_rate: Option<f32>\n-biquad_filters: [BiquadFilter; BANDS]\n-x1, x2, y1, y2: [f32: CHANNELS]",
			"rawText": "-sample_rate: Option<f32>\n-biquad_filters: [BiquadFilter; BANDS]\n-x1, x2, y1, y2: [f32: CHANNELS]",
			"baseline": 53,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "INAa1si8v5qCQT8OJCBjm",
			"originalText": "-sample_rate: Option<f32>\n-biquad_filters: [BiquadFilter; BANDS]\n-x1, x2, y1, y2: [f32: CHANNELS]"
		},
		{
			"type": "text",
			"version": 1435,
			"versionNonce": 1103540739,
			"isDeleted": false,
			"id": "p0fNJLeP",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 438.63736426688865,
			"y": 809.4943554417785,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 685,
			"height": 57,
			"seed": 1050272653,
			"groupIds": [
				"wyW9HJRNi-C415tS2W48D"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<BANDS>)\n+set_sample_rate(&mut self, sample_rate: f32)\n+default() -> Self",
			"rawText": "+process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<BANDS>)\n+set_sample_rate(&mut self, sample_rate: f32)\n+default() -> Self",
			"baseline": 53,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "LDbw2efYTF0EN5eR2hzEe",
			"originalText": "+process(&mut self, buffer: &mut Buffer, params: &EqualizerParams<BANDS>)\n+set_sample_rate(&mut self, sample_rate: f32)\n+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 1139,
			"versionNonce": 683123629,
			"isDeleted": false,
			"id": "odnCji_nGIse-B0beIpWb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 489.04323286173417,
			"y": 981.5386115418687,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 355.6377828387916,
			"height": 198.6347440459267,
			"seed": 1669717955,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 979,
			"versionNonce": 449280419,
			"isDeleted": false,
			"id": "nkCCHaKJ4HjuCqy4VjyB3",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 489.7503225740695,
			"y": 1086.940160034569,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 352.5016714062672,
			"height": 0,
			"seed": 1746612717,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					176.2508357031336,
					0
				],
				[
					352.5016714062672,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1290,
			"versionNonce": 1925228045,
			"isDeleted": false,
			"id": "K8xQ6nK54i-2Q4_62Rjc2",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 495.6062899760807,
			"y": 1008.1158167929821,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 342,
			"height": 44,
			"seed": 534441827,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "9DUsiLiT",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1244,
			"versionNonce": 328092995,
			"isDeleted": false,
			"id": "9DUsiLiT",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 542.6062899760807,
			"y": 1013.1158167929821,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 248,
			"height": 34,
			"seed": 320367693,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "EqualizerParams",
			"rawText": "EqualizerParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "K8xQ6nK54i-2Q4_62Rjc2",
			"originalText": "EqualizerParams"
		},
		{
			"type": "rectangle",
			"version": 1581,
			"versionNonce": 1832541293,
			"isDeleted": false,
			"id": "qF6pAgzo9mWY9Bbx6Ykns",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 493.5234085037539,
			"y": 1088.6483142900215,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 339,
			"height": 48,
			"seed": 439159555,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "P84bhVxS",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 936,
			"versionNonce": 1601809635,
			"isDeleted": false,
			"id": "Ry00WgiknbGn5ZuftZo6S",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 489.0389678768704,
			"y": 1142.8621973608542,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 354.38682764331827,
			"height": 0,
			"seed": 859789997,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					177.19341382165916,
					0
				],
				[
					354.3868276433183,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1477,
			"versionNonce": 804144845,
			"isDeleted": false,
			"id": "g7LB1_27d3EWBfLtSZKO9",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 494.1965654394089,
			"y": 1147.5743823643766,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 341,
			"height": 29,
			"seed": 653963939,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "eoXonYce",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1365,
			"versionNonce": 1108875395,
			"isDeleted": false,
			"id": "P84bhVxS",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 498.5234085037539,
			"y": 1093.6483142900215,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 254,
			"height": 38,
			"seed": 1631157517,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+bands: [BandParams; BANDS]\n+bypass: BoolParam",
			"rawText": "+bands: [BandParams; BANDS]\n+bypass: BoolParam",
			"baseline": 34,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "qF6pAgzo9mWY9Bbx6Ykns",
			"originalText": "+bands: [BandParams; BANDS]\n+bypass: BoolParam"
		},
		{
			"type": "text",
			"version": 1150,
			"versionNonce": 494265645,
			"isDeleted": false,
			"id": "eoXonYce",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 499.1965654394089,
			"y": 1152.5743823643766,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 171,
			"height": 19,
			"seed": 116549187,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+default() -> Self",
			"rawText": "+default() -> Self",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "g7LB1_27d3EWBfLtSZKO9",
			"originalText": "+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 2147,
			"versionNonce": 153620515,
			"isDeleted": false,
			"id": "ncHuF_bQFHcJhQX_WSZxz",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 494.0540094048599,
			"y": 986.2061039712493,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 342,
			"height": 29,
			"seed": 1686131565,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "R6alQ75K",
					"type": "text"
				}
			],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2134,
			"versionNonce": 154843021,
			"isDeleted": false,
			"id": "R6alQ75K",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 584.5540094048599,
			"y": 991.2061039712493,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 1626514915,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ncHuF_bQFHcJhQX_WSZxz",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "text",
			"version": 175,
			"versionNonce": 1184371651,
			"isDeleted": false,
			"id": "o4bTWuvb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 547.4022785347383,
			"y": 1053.0201962748602,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 236,
			"height": 24,
			"seed": 1998364653,
			"groupIds": [
				"W7tXtn2N7JDbJAYAnr87T"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "<const BANDS: usize>",
			"rawText": "<const BANDS: usize>",
			"baseline": 19,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "<const BANDS: usize>"
		},
		{
			"type": "rectangle",
			"version": 1040,
			"versionNonce": 494288365,
			"isDeleted": false,
			"id": "uH_K1_pm9EwDHjuoVI6dQ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 485.6019938685786,
			"y": 1291.5193636629497,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 827.0988023999663,
			"height": 251.1383097810862,
			"seed": 408348941,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476627,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 805,
			"versionNonce": 1250337635,
			"isDeleted": false,
			"id": "Ys_WCg38qG9xt5roOTiAf",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 487.4101152227772,
			"y": 1366.92091215565,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 824.9052690859672,
			"height": 0,
			"seed": 1806809667,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					412.4526345429836,
					0
				],
				[
					824.9052690859672,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1162,
			"versionNonce": 20052045,
			"isDeleted": false,
			"id": "aw352xER6Y4VC65E8nRmP",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 492.3235045062629,
			"y": 1318.096568914063,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 818,
			"height": 44,
			"seed": 756039533,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "Yso3XsNd",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1132,
			"versionNonce": 1564622595,
			"isDeleted": false,
			"id": "Yso3XsNd",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 818.3235045062629,
			"y": 1323.096568914063,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 166,
			"height": 34,
			"seed": 447183331,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "BandParams",
			"rawText": "BandParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "aw352xER6Y4VC65E8nRmP",
			"originalText": "BandParams"
		},
		{
			"type": "rectangle",
			"version": 1483,
			"versionNonce": 1044866733,
			"isDeleted": false,
			"id": "ieZQ1R1EAKtraKNfCGj1t",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 491.1832011524616,
			"y": 1368.6290664111025,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 326,
			"height": 107,
			"seed": 1628424653,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "9CwkSl57",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 841,
			"versionNonce": 961111715,
			"isDeleted": false,
			"id": "4N1t3jCRfnBcl_yX_Lb9g",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 486.6987605255781,
			"y": 1481.8429494819356,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 823.9669846915986,
			"height": 0,
			"seed": 1342457219,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					411.9834923457993,
					0
				],
				[
					823.9669846915986,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1524,
			"versionNonce": 1734347021,
			"isDeleted": false,
			"id": "dzOuY43V2VYo4LyGc2XDD",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 491.8563580881164,
			"y": 1487.5551344854575,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 819,
			"height": 53,
			"seed": 1433339949,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tM036NAn",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1343,
			"versionNonce": 1162517059,
			"isDeleted": false,
			"id": "9CwkSl57",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 496.1832011524616,
			"y": 1373.6290664111025,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 292,
			"height": 95,
			"seed": 1124506915,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+band_type: EnumParam<BandType>\n+frequency: FloatParam\n+q: FloatParam\n+gain: FloatParam\n-dirty: Arc<AtomicBool>",
			"rawText": "+band_type: EnumParam<BandType>\n+frequency: FloatParam\n+q: FloatParam\n+gain: FloatParam\n-dirty: Arc<AtomicBool>",
			"baseline": 91,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "ieZQ1R1EAKtraKNfCGj1t",
			"originalText": "+band_type: EnumParam<BandType>\n+frequency: FloatParam\n+q: FloatParam\n+gain: FloatParam\n-dirty: Arc<AtomicBool>"
		},
		{
			"type": "text",
			"version": 1201,
			"versionNonce": 2113232749,
			"isDeleted": false,
			"id": "tM036NAn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 496.8563580881164,
			"y": 1492.5551344854575,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 807,
			"height": 38,
			"seed": 1276607117,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self\n-calculate_filter(&self, sample_rate: f32) -> BiquadFilter",
			"rawText": "-new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self\n-calculate_filter(&self, sample_rate: f32) -> BiquadFilter",
			"baseline": 34,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "dzOuY43V2VYo4LyGc2XDD",
			"originalText": "-new(band_number: i32, band_type: BandType, frequency: f32, q: f32, gain: f32) -> Self\n-calculate_filter(&self, sample_rate: f32) -> BiquadFilter"
		},
		{
			"type": "rectangle",
			"version": 2118,
			"versionNonce": 237263331,
			"isDeleted": false,
			"id": "nqYXV3Edul-9HcUC9rw8i",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 491.71380205356763,
			"y": 1296.1868560923303,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 818,
			"height": 32,
			"seed": 1058865347,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "4kC3drPb",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2104,
			"versionNonce": 407955917,
			"isDeleted": false,
			"id": "4kC3drPb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 820.2138020535676,
			"y": 1302.6868560923303,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 230858989,
			"groupIds": [
				"KDBP2Pr6b57ry7ZkEzi_L"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "nqYXV3Edul-9HcUC9rw8i",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "rectangle",
			"version": 952,
			"versionNonce": 15017347,
			"isDeleted": false,
			"id": "LrNxMHFCbbN_7rRrnrNr0",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 965.4150647199731,
			"y": 963.2019462224791,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 335.7934922157535,
			"height": 231.60648599700008,
			"seed": 495571523,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 775,
			"versionNonce": 1200367661,
			"isDeleted": false,
			"id": "Gpkfum6ZiwllCVy9awv9q",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 966.0300244738537,
			"y": 1068.4325347231306,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 334.7931205020727,
			"height": 0,
			"seed": 912631661,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					167.39656025103636,
					0
				],
				[
					334.7931205020727,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1116,
			"versionNonce": 1691057443,
			"isDeleted": false,
			"id": "mgnxXw0VduvCbIgxrfzni",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 972.1365753576574,
			"y": 1016.3670907585715,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 323,
			"height": 44,
			"seed": 163353059,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "azlC8eTN",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1080,
			"versionNonce": 1032886925,
			"isDeleted": false,
			"id": "azlC8eTN",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1067.1365753576574,
			"y": 1021.3670907585715,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 133,
			"height": 34,
			"seed": 1803039181,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "BandType",
			"rawText": "BandType",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "mgnxXw0VduvCbIgxrfzni",
			"originalText": "BandType"
		},
		{
			"type": "rectangle",
			"version": 1448,
			"versionNonce": 356901059,
			"isDeleted": false,
			"id": "K6LQkMOyoQAAOBeoVJ6Tn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 968.60994880322,
			"y": 1068.947527378265,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 150,
			"height": 124,
			"seed": 1287019907,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tcox6TkE",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1220,
			"versionNonce": 857194733,
			"isDeleted": false,
			"id": "tcox6TkE",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 973.60994880322,
			"y": 1073.947527378265,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 95,
			"height": 114,
			"seed": 1434837645,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+Peak\n+Notch\n+HighShelf\n+LowShelf\n+HighPass\n+LowPass",
			"rawText": "+Peak\n+Notch\n+HighShelf\n+LowShelf\n+HighPass\n+LowPass",
			"baseline": 110,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "K6LQkMOyoQAAOBeoVJ6Tn",
			"originalText": "+Peak\n+Notch\n+HighShelf\n+LowShelf\n+HighPass\n+LowPass"
		},
		{
			"type": "rectangle",
			"version": 2031,
			"versionNonce": 2079510627,
			"isDeleted": false,
			"id": "6A7Df2s7qpRK-PJW8OsRT",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 971.5268729049624,
			"y": 967.8694386518597,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 325,
			"height": 29,
			"seed": 757336301,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "FbJV8KoC",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2026,
			"versionNonce": 229317453,
			"isDeleted": false,
			"id": "FbJV8KoC",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1095.5268729049624,
			"y": 972.8694386518597,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 77,
			"height": 19,
			"seed": 766445667,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<enum>>",
			"rawText": "<<enum>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "6A7Df2s7qpRK-PJW8OsRT",
			"originalText": "<<enum>>"
		},
		{
			"type": "rectangle",
			"version": 2095,
			"versionNonce": 1252051971,
			"isDeleted": false,
			"id": "t9PPTWS2o0z433GNjfX-5",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 970.5835676431157,
			"y": 994.3018164025327,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 324,
			"height": 29,
			"seed": 1058382051,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "dJ2pGwAg",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2123,
			"versionNonce": 1168386477,
			"isDeleted": false,
			"id": "dJ2pGwAg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 977.0835676431157,
			"y": 999.3018164025327,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 311,
			"height": 19,
			"seed": 523997901,
			"groupIds": [
				"jmVbcjiSysY5lBdSc8iNO"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Enum, Debug, PartialEq)]",
			"rawText": "#[derive(Enum, Debug, PartialEq)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "t9PPTWS2o0z433GNjfX-5",
			"originalText": "#[derive(Enum, Debug, PartialEq)]"
		},
		{
			"type": "rectangle",
			"version": 774,
			"versionNonce": 1968067491,
			"isDeleted": false,
			"id": "6Q5K8V2uvYN0GFGXOFguW",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1544.3673581688959,
			"y": 571.1831062217782,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 878,
			"height": 833,
			"seed": 155727651,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "GUPO0U47",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 555,
			"versionNonce": 204237837,
			"isDeleted": false,
			"id": "GUPO0U47",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1870.8673581688959,
			"y": 576.1831062217782,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 225,
			"height": 24,
			"seed": 936673421,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "frequency_sidechain",
			"rawText": "frequency_sidechain",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "6Q5K8V2uvYN0GFGXOFguW",
			"originalText": "frequency_sidechain"
		},
		{
			"type": "rectangle",
			"version": 1090,
			"versionNonce": 1814685507,
			"isDeleted": false,
			"id": "1PZtidqcLoPw52BYLmnff",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1633.6013929110577,
			"y": 675.7249908171378,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 732.6640647423833,
			"height": 410.19593787741326,
			"seed": 1930124227,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 722,
			"versionNonce": 826106477,
			"isDeleted": false,
			"id": "NcyppOeFg-QdYlUAr5EkH",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1634.4183864230477,
			"y": 737.9792749720355,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 731.128753782757,
			"height": 0,
			"seed": 448493037,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					365.5643768913785,
					0
				],
				[
					731.128753782757,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1270,
			"versionNonce": 361891555,
			"isDeleted": false,
			"id": "baPf7EkmL9JUV7EF92SYs",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1640.2127163710506,
			"y": 679.7995821860602,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 719,
			"height": 52,
			"seed": 424733539,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "vulOTFL3",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1209,
			"versionNonce": 1235258573,
			"isDeleted": false,
			"id": "vulOTFL3",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1851.2127163710506,
			"y": 688.7995821860602,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 297,
			"height": 34,
			"seed": 286273613,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FrequencySidechain",
			"rawText": "FrequencySidechain",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "baPf7EkmL9JUV7EF92SYs",
			"originalText": "FrequencySidechain"
		},
		{
			"type": "rectangle",
			"version": 1477,
			"versionNonce": 135606915,
			"isDeleted": false,
			"id": "sS0ZXqIfodGnJ53d12ODc",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1641.267356195543,
			"y": 743.8391969131098,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 446,
			"height": 238,
			"seed": 1803781891,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "fP7ZoJJI",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 860,
			"versionNonce": 351234861,
			"isDeleted": false,
			"id": "N-yeM4Lq8F4F5OvMHFiDO",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1635.782915568659,
			"y": 966.0355893123123,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 728.0166636630272,
			"height": 0,
			"seed": 631152301,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					364.0083318315136,
					0
				],
				[
					728.0166636630272,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1697,
			"versionNonce": 74574371,
			"isDeleted": false,
			"id": "eLY9pILqwlrC55krknD8D",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1637.732771361841,
			"y": 971.7208395421317,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 726,
			"height": 105,
			"seed": 2207395,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "LlntZxYQ",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1789,
			"versionNonce": 596868493,
			"isDeleted": false,
			"id": "fP7ZoJJI",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1646.267356195543,
			"y": 748.8391969131098,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 432,
			"height": 209,
			"seed": 1326465293,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-stft: StftHelper<1>\n-channels: usize\n-window_size: usize\n-overlap_times: usize\n-gain_compensation: f32\n-forward_fft: ForwardFFT<f32>\n-inverse_fft: InverseFFT<f32>\n-main_complex_buffer: Vec<Complex32>\n-sidechain_complex_buffer: Vec<Vec<Complex32>>\n-window_function: Vec<f32>\n-smoother: Vec<Vec<Smoother>>",
			"rawText": "-stft: StftHelper<1>\n-channels: usize\n-window_size: usize\n-overlap_times: usize\n-gain_compensation: f32\n-forward_fft: ForwardFFT<f32>\n-inverse_fft: InverseFFT<f32>\n-main_complex_buffer: Vec<Complex32>\n-sidechain_complex_buffer: Vec<Vec<Complex32>>\n-window_function: Vec<f32>\n-smoother: Vec<Vec<Smoother>>",
			"baseline": 205,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "sS0ZXqIfodGnJ53d12ODc",
			"originalText": "-stft: StftHelper<1>\n-channels: usize\n-window_size: usize\n-overlap_times: usize\n-gain_compensation: f32\n-forward_fft: ForwardFFT<f32>\n-inverse_fft: InverseFFT<f32>\n-main_complex_buffer: Vec<Complex32>\n-sidechain_complex_buffer: Vec<Vec<Complex32>>\n-window_function: Vec<f32>\n-smoother: Vec<Vec<Smoother>>"
		},
		{
			"type": "text",
			"version": 1368,
			"versionNonce": 798404035,
			"isDeleted": false,
			"id": "LlntZxYQ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1642.732771361841,
			"y": 976.7208395421317,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 714,
			"height": 95,
			"seed": 433709635,
			"groupIds": [
				"V5jKjf0N1-86-o2Ycz09f"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+new(channels: usize, window_size: usize, hop_size: usize) -> Self\n+process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer,\n    sample_rate: f32, params: &FrequencySidechainParams)\n+latency_samples(&self) -> u32\n+reset(&mut self)",
			"rawText": "+new(channels: usize, window_size: usize, hop_size: usize) -> Self\n+process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer,\n    sample_rate: f32, params: &FrequencySidechainParams)\n+latency_samples(&self) -> u32\n+reset(&mut self)",
			"baseline": 91,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "eLY9pILqwlrC55krknD8D",
			"originalText": "+new(channels: usize, window_size: usize, hop_size: usize) -> Self\n+process(&mut self, main_buffer: &mut Buffer, sidechain_buffer: &mut Buffer,\n    sample_rate: f32, params: &FrequencySidechainParams)\n+latency_samples(&self) -> u32\n+reset(&mut self)"
		},
		{
			"type": "rectangle",
			"version": 1036,
			"versionNonce": 1019626477,
			"isDeleted": false,
			"id": "Om8DjR4QfYjtkb7q5Nals",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1770.1078876603747,
			"y": 1147.3992438433093,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 452.72332904692166,
			"height": 187.1314899723125,
			"seed": 1605611437,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 800,
			"versionNonce": 211942755,
			"isDeleted": false,
			"id": "ie8TTUmlcqwSSZPreosYm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1771.9160090145733,
			"y": 1222.8007923360096,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 450.5297957329228,
			"height": 0,
			"seed": 1784819107,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					225.2648978664614,
					0
				],
				[
					450.5297957329228,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1136,
			"versionNonce": 76627533,
			"isDeleted": false,
			"id": "m1sSIPCEKSCIFir1Vctbq",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1776.829398298059,
			"y": 1173.9764490944226,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 437,
			"height": 44,
			"seed": 106639885,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "7otgJBOs",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1120,
			"versionNonce": 920803587,
			"isDeleted": false,
			"id": "7otgJBOs",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1797.329398298059,
			"y": 1178.9764490944226,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 396,
			"height": 34,
			"seed": 95584579,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FrequencySidechainParams",
			"rawText": "FrequencySidechainParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "m1sSIPCEKSCIFir1Vctbq",
			"originalText": "FrequencySidechainParams"
		},
		{
			"type": "rectangle",
			"version": 1473,
			"versionNonce": 1640885421,
			"isDeleted": false,
			"id": "xOSoA7rJq5dfEpR0gg4Rp",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1775.6890949442577,
			"y": 1224.508946591462,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 444,
			"height": 107,
			"seed": 1168337005,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "AXGlc8Sy",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 819,
			"versionNonce": 1540767907,
			"isDeleted": false,
			"id": "nAwyq4X4l7DDhLBPr4kqq",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1771.2046543173742,
			"y": 1294.7228296622948,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 450.5297957329228,
			"height": 0,
			"seed": 1727488227,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					225.2648978664614,
					0
				],
				[
					450.5297957329228,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1366,
			"versionNonce": 606457613,
			"isDeleted": false,
			"id": "KmyIqROxYFa-ha_H3Gdge",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1776.362251879913,
			"y": 1298.435014665817,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 438,
			"height": 30,
			"seed": 794153677,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "lPGcrI1j",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1292,
			"versionNonce": 1965831235,
			"isDeleted": false,
			"id": "AXGlc8Sy",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1780.6890949442577,
			"y": 1229.508946591462,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 236,
			"height": 57,
			"seed": 2009025667,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+detail: FloatParam\n+precision: FloatParam\n+smoother: SmootherParams",
			"rawText": "+detail: FloatParam\n+precision: FloatParam\n+smoother: SmootherParams",
			"baseline": 53,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "xOSoA7rJq5dfEpR0gg4Rp",
			"originalText": "+detail: FloatParam\n+precision: FloatParam\n+smoother: SmootherParams"
		},
		{
			"type": "text",
			"version": 1075,
			"versionNonce": 1779590509,
			"isDeleted": false,
			"id": "lPGcrI1j",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1781.362251879913,
			"y": 1303.435014665817,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 171,
			"height": 19,
			"seed": 1611395373,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+default() -> Self",
			"rawText": "+default() -> Self",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "KmyIqROxYFa-ha_H3Gdge",
			"originalText": "+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 2082,
			"versionNonce": 1120056291,
			"isDeleted": false,
			"id": "e1BRGBFsR89sD1YBTE_Fe",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1776.2196958453637,
			"y": 1152.06673627269,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 440,
			"height": 31,
			"seed": 1808319523,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "7sM2dvIX",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2069,
			"versionNonce": 1188900813,
			"isDeleted": false,
			"id": "7sM2dvIX",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1915.7196958453637,
			"y": 1158.06673627269,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 623854477,
			"groupIds": [
				"PvEE-VlDhhQvcbEg7PK7F"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "e1BRGBFsR89sD1YBTE_Fe",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "rectangle",
			"version": 879,
			"versionNonce": 78135171,
			"isDeleted": false,
			"id": "DXfJ-2KPGq1H488m2eIy-",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2529.95571203955,
			"y": 579.7008887211273,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 878,
			"height": 586,
			"seed": 650269165,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "3dzTBa6a",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 640,
			"versionNonce": 681553453,
			"isDeleted": false,
			"id": "3dzTBa6a",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2920.95571203955,
			"y": 584.7008887211273,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 96,
			"height": 24,
			"seed": 1521864547,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "smoother",
			"rawText": "smoother",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "DXfJ-2KPGq1H488m2eIy-",
			"originalText": "smoother"
		},
		{
			"type": "rectangle",
			"version": 1235,
			"versionNonce": 1964726051,
			"isDeleted": false,
			"id": "sa6Cjlnzx30MAx-rNr-_w",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2788.7189523879874,
			"y": 907.447824207999,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 272.66063384645804,
			"height": 205.72492045496904,
			"seed": 800423789,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 953,
			"versionNonce": 2118344845,
			"isDeleted": false,
			"id": "qin41lwrKQHZW-FqvKmUL",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2790.5270737421856,
			"y": 982.8493727006993,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 270.46710053245965,
			"height": 0,
			"seed": 1878407651,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					135.23355026622983,
					0
				],
				[
					270.46710053245965,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1355,
			"versionNonce": 42629827,
			"isDeleted": false,
			"id": "NELYGZDfQ-vjCPvMMlm4H",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2795.4404630256718,
			"y": 934.0250294591121,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 264,
			"height": 44,
			"seed": 1561135565,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "zojWDKOv",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1356,
			"versionNonce": 495044333,
			"isDeleted": false,
			"id": "zojWDKOv",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2811.4404630256718,
			"y": 939.0250294591121,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 232,
			"height": 34,
			"seed": 226166147,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "SmootherParams",
			"rawText": "SmootherParams",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "NELYGZDfQ-vjCPvMMlm4H",
			"originalText": "SmootherParams"
		},
		{
			"type": "rectangle",
			"version": 1671,
			"versionNonce": 1234446947,
			"isDeleted": false,
			"id": "TCokawgIjdAmaNT4b7Kv8",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2794.30015967187,
			"y": 984.5575269561518,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 262,
			"height": 86,
			"seed": 1606699053,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "YdCW6qFn",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 985,
			"versionNonce": 1759797581,
			"isDeleted": false,
			"id": "jHJscr7sOrzWnkWUjurms",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2789.815719044987,
			"y": 1072.7714100269848,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 272.42430374116043,
			"height": 0,
			"seed": 598039843,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					136.21215187058021,
					0
				],
				[
					272.42430374116043,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1566,
			"versionNonce": 1325852163,
			"isDeleted": false,
			"id": "xrgUZqoWmg_sSEv9Diriq",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2793.9947150031753,
			"y": 1077.8259693609014,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 260,
			"height": 29,
			"seed": 1506065037,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "q2cHqNvg",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1529,
			"versionNonce": 902082477,
			"isDeleted": false,
			"id": "YdCW6qFn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2799.30015967187,
			"y": 989.5575269561518,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 235,
			"height": 76,
			"seed": 501465283,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+attack_bypass: BoolParam\n+attack_speed: FloatParam\n+decay_bypass: BoolParam\n+decay_speed: FloatParam",
			"rawText": "+attack_bypass: BoolParam\n+attack_speed: FloatParam\n+decay_bypass: BoolParam\n+decay_speed: FloatParam",
			"baseline": 72,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "TCokawgIjdAmaNT4b7Kv8",
			"originalText": "+attack_bypass: BoolParam\n+attack_speed: FloatParam\n+decay_bypass: BoolParam\n+decay_speed: FloatParam"
		},
		{
			"type": "text",
			"version": 1231,
			"versionNonce": 2044800419,
			"isDeleted": false,
			"id": "q2cHqNvg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2798.9947150031753,
			"y": 1082.8259693609014,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 171,
			"height": 19,
			"seed": 1942479085,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+default() -> Self",
			"rawText": "+default() -> Self",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "xrgUZqoWmg_sSEv9Diriq",
			"originalText": "+default() -> Self"
		},
		{
			"type": "rectangle",
			"version": 2252,
			"versionNonce": 1917596173,
			"isDeleted": false,
			"id": "JhLjz3Ky2Ct22PVcnCtwd",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2794.8307605729765,
			"y": 912.1153166373795,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 262,
			"height": 31,
			"seed": 1976873059,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "f66qOwQW",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2238,
			"versionNonce": 1297187139,
			"isDeleted": false,
			"id": "f66qOwQW",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2845.3307605729765,
			"y": 918.1153166373795,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 161,
			"height": 19,
			"seed": 56048461,
			"groupIds": [
				"KAVOfj82Rx-AhPooSD3mB"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Params)]",
			"rawText": "#[derive(Params)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "JhLjz3Ky2Ct22PVcnCtwd",
			"originalText": "#[derive(Params)]"
		},
		{
			"type": "rectangle",
			"version": 1342,
			"versionNonce": 192178285,
			"isDeleted": false,
			"id": "DMJgtuJu-x4IZProKOlFv",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2579.14552555613,
			"y": 694.0297231953041,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 760.5752539986623,
			"height": 149.42707966817625,
			"seed": 1243596355,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1013,
			"versionNonce": 1656119523,
			"isDeleted": false,
			"id": "YJQ04g5axdEzk4W065-YF",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2580.953646910328,
			"y": 768.4435902706921,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 757.3940392673517,
			"height": 0,
			"seed": 1029239661,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					378.69701963367584,
					0
				],
				[
					757.3940392673517,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1405,
			"versionNonce": 319214285,
			"isDeleted": false,
			"id": "q3XmgPiGWA5FF4F_MgbSv",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2586.8547176111265,
			"y": 719.619247029105,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 747,
			"height": 44,
			"seed": 393479651,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "3DEUtDeP",
					"type": "text"
				}
			],
			"updated": 1734013476628,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1414,
			"versionNonce": 597576835,
			"isDeleted": false,
			"id": "3DEUtDeP",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2893.8547176111265,
			"y": 724.619247029105,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 133,
			"height": 34,
			"seed": 1310850509,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476628,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Smoother",
			"rawText": "Smoother",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "q3XmgPiGWA5FF4F_MgbSv",
			"originalText": "Smoother"
		},
		{
			"type": "rectangle",
			"version": 1772,
			"versionNonce": 631042349,
			"isDeleted": false,
			"id": "r7yu2nBEAWCcqusJQcEHZ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2583.739051422701,
			"y": 771.1394259434569,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 264,
			"height": 29,
			"seed": 459231619,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "BCfEhxIF",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1071,
			"versionNonce": 1137165347,
			"isDeleted": false,
			"id": "KhKXzysFVFY8rpzzBr2_f",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2581.2299736304417,
			"y": 802.3656275969774,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 757.3758796414281,
			"height": 0,
			"seed": 1243266093,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					378.68793982071406,
					0
				],
				[
					757.3758796414281,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1728,
			"versionNonce": 278938509,
			"isDeleted": false,
			"id": "GPvOXX4Szana66c284c5t",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2585.40896958863,
			"y": 808.420186930894,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 753,
			"height": 29,
			"seed": 119548195,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "S83fr9eX",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1618,
			"versionNonce": 2065778627,
			"isDeleted": false,
			"id": "BCfEhxIF",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2588.739051422701,
			"y": 776.1394259434569,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 255,
			"height": 19,
			"seed": 422358669,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-current_value: Option<f32>",
			"rawText": "-current_value: Option<f32>",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "r7yu2nBEAWCcqusJQcEHZ",
			"originalText": "-current_value: Option<f32>"
		},
		{
			"type": "text",
			"version": 1370,
			"versionNonce": 944392685,
			"isDeleted": false,
			"id": "S83fr9eX",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2590.40896958863,
			"y": 813.420186930894,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 743,
			"height": 19,
			"seed": 1613040835,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+process(&mut self, value: &mut f32, sample_rate: f32, params: &SmootherParams)",
			"rawText": "+process(&mut self, value: &mut f32, sample_rate: f32, params: &SmootherParams)",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "GPvOXX4Szana66c284c5t",
			"originalText": "+process(&mut self, value: &mut f32, sample_rate: f32, params: &SmootherParams)"
		},
		{
			"type": "rectangle",
			"version": 2327,
			"versionNonce": 2143901539,
			"isDeleted": false,
			"id": "oOx7djO8gw6idvZ7awQhE",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2586.245015158431,
			"y": 697.7095342073724,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 751,
			"height": 32,
			"seed": 1779520749,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "kQhqgKQe",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2332,
			"versionNonce": 1034191949,
			"isDeleted": false,
			"id": "kQhqgKQe",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2843.745015158431,
			"y": 704.2095342073724,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 236,
			"height": 19,
			"seed": 2047638627,
			"groupIds": [
				"AnTE8IiHuw81mE3_DQD8C"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Default, Clone)]",
			"rawText": "#[derive(Default, Clone)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "oOx7djO8gw6idvZ7awQhE",
			"originalText": "#[derive(Default, Clone)]"
		},
		{
			"type": "rectangle",
			"version": 1394,
			"versionNonce": 1074688771,
			"isDeleted": false,
			"id": "8oSzxNWKf0NaqDw83l_yS",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2583.253970962,
			"y": 92.0544647119516,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 772.910510111519,
			"height": 146.37155772111637,
			"seed": 1145469027,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1236,
			"versionNonce": 1686663853,
			"isDeleted": false,
			"id": "t46VedXVqDsLgkWON9PdS",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2585.0620923161987,
			"y": 166.54692229556122,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 770.6834895880856,
			"height": 0,
			"seed": 1673917261,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					385.3417447940428,
					0
				],
				[
					770.6834895880856,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1551,
			"versionNonce": 1550785187,
			"isDeleted": false,
			"id": "YQ5zpB5CA0vCjKK8uzIiB",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2590.91805971821,
			"y": 117.72257905397419,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 762,
			"height": 44,
			"seed": 440366083,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "IfgzOHqG",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1492,
			"versionNonce": 966759693,
			"isDeleted": false,
			"id": "IfgzOHqG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2872.41805971821,
			"y": 122.72257905397419,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 199,
			"height": 34,
			"seed": 1118044589,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "BiquadFilter",
			"rawText": "BiquadFilter",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "YQ5zpB5CA0vCjKK8uzIiB",
			"originalText": "BiquadFilter"
		},
		{
			"type": "rectangle",
			"version": 1825,
			"versionNonce": 1637469763,
			"isDeleted": false,
			"id": "pHrYOkYp8NrpbBhkXrRXB",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2588.835178245883,
			"y": 168.25507655101364,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 275,
			"height": 29,
			"seed": 718797731,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "EDShATxL",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1201,
			"versionNonce": 2019781485,
			"isDeleted": false,
			"id": "lG0JvJzVwic6jAlx656sG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2584.3507376189996,
			"y": 200.46895962184635,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 770.7504640069548,
			"height": 0,
			"seed": 191972365,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					385.3752320034774,
					0
				],
				[
					770.7504640069548,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1792,
			"versionNonce": 1114443235,
			"isDeleted": false,
			"id": "vFhfcwzeoZiIZofpXPBpf",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2589.508335181538,
			"y": 203.2720537162777,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 764,
			"height": 29,
			"seed": 306089795,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "ULcj07L1",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1555,
			"versionNonce": 824014285,
			"isDeleted": false,
			"id": "EDShATxL",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2593.835178245883,
			"y": 173.25507655101364,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 227,
			"height": 19,
			"seed": 572996205,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+b0, b1, b2, a1, a2: f32",
			"rawText": "+b0, b1, b2, a1, a2: f32",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "pHrYOkYp8NrpbBhkXrRXB",
			"originalText": "+b0, b1, b2, a1, a2: f32"
		},
		{
			"type": "text",
			"version": 1461,
			"versionNonce": 301422979,
			"isDeleted": false,
			"id": "ULcj07L1",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2594.508335181538,
			"y": 208.2720537162777,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 752,
			"height": 19,
			"seed": 1657908963,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+biquad_transform(&self, sample: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> f32",
			"rawText": "+biquad_transform(&self, sample: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> f32",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "vFhfcwzeoZiIZofpXPBpf",
			"originalText": "+biquad_transform(&self, sample: f32, x1: f32, x2: f32, y1: f32, y2: f32) -> f32"
		},
		{
			"type": "rectangle",
			"version": 2503,
			"versionNonce": 786904109,
			"isDeleted": false,
			"id": "mExja8k9G_BIq2HpJ_6AH",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2589.365779146989,
			"y": 95.8128662322415,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 763,
			"height": 32,
			"seed": 797926605,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "XVeGAe9G",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2522,
			"versionNonce": 2104963363,
			"isDeleted": false,
			"id": "XVeGAe9G",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2791.865779146989,
			"y": 102.3128662322415,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 358,
			"height": 19,
			"seed": 1595923075,
			"groupIds": [
				"78R5Ctqo4OubKx-8kj9JH"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Debug, Default, Copy, Clone)]",
			"rawText": "#[derive(Debug, Default, Copy, Clone)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "mExja8k9G_BIq2HpJ_6AH",
			"originalText": "#[derive(Debug, Default, Copy, Clone)]"
		},
		{
			"type": "rectangle",
			"version": 1426,
			"versionNonce": 497690253,
			"isDeleted": false,
			"id": "YYWrsr1Iog7N79KbO0P53",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2591.0687289501952,
			"y": -805.1117322219866,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 772.910510111519,
			"height": 110.07589225567418,
			"seed": 1873504451,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1246,
			"versionNonce": 106632387,
			"isDeleted": false,
			"id": "ty3CJLawIF5NrN1Ql-2bs",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2592.8768503043943,
			"y": -730.6192746383767,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 770.6834895880856,
			"height": 0,
			"seed": 1101969645,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					385.3417447940428,
					0
				],
				[
					770.6834895880856,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1563,
			"versionNonce": 1881132269,
			"isDeleted": false,
			"id": "NUyBTiJHbIVNbXm1iRuZA",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2598.732817706405,
			"y": -779.4436178799642,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 762,
			"height": 44,
			"seed": 6772835,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "HyoCW4im",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1516,
			"versionNonce": 2084668515,
			"isDeleted": false,
			"id": "HyoCW4im",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2888.732817706405,
			"y": -774.4436178799642,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 182,
			"height": 34,
			"seed": 514003789,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "BufferUtils",
			"rawText": "BufferUtils",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "NUyBTiJHbIVNbXm1iRuZA",
			"originalText": "BufferUtils"
		},
		{
			"type": "rectangle",
			"version": 1811,
			"versionNonce": 97625933,
			"isDeleted": false,
			"id": "_2f4o5lfc6LyJhUnzhguQ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2597.323093169733,
			"y": -728.8941432176607,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 764,
			"height": 29,
			"seed": 1010883491,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "TkD3AmMN",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1579,
			"versionNonce": 922288131,
			"isDeleted": false,
			"id": "TkD3AmMN",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2602.323093169733,
			"y": -723.8941432176607,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 696,
			"height": 19,
			"seed": 1133211459,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+on_each_sample<F>(&mut self, f: F) where F: FnMut(usize, usize, &mut f32)",
			"rawText": "+on_each_sample<F>(&mut self, f: F) where F: FnMut(usize, usize, &mut f32)",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "_2f4o5lfc6LyJhUnzhguQ",
			"originalText": "+on_each_sample<F>(&mut self, f: F) where F: FnMut(usize, usize, &mut f32)"
		},
		{
			"type": "rectangle",
			"version": 2516,
			"versionNonce": 286864813,
			"isDeleted": false,
			"id": "ui90UQufRqOkRUds7UTzE",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2597.1805371351843,
			"y": -801.3533307016969,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 763,
			"height": 32,
			"seed": 546855533,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "ugkmWwXs",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2544,
			"versionNonce": 276826019,
			"isDeleted": false,
			"id": "ugkmWwXs",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2935.6805371351843,
			"y": -794.8533307016969,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 394432227,
			"groupIds": [
				"MWtHBCqTsnbYk_zeuaBlK"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ui90UQufRqOkRUds7UTzE",
			"originalText": "<<trait>>"
		},
		{
			"type": "rectangle",
			"version": 1574,
			"versionNonce": 1025505293,
			"isDeleted": false,
			"id": "ClFHxOAdAQhUIwnbesiIk",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2680.0285303353444,
			"y": -571.2243882067964,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 581.0825011551133,
			"height": 165.44892576886346,
			"seed": 715615021,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "GIqzQrHCD5pducA_UZqIc",
					"type": "arrow"
				},
				{
					"id": "jfKj0FtGcfJ0CnHmOecmd",
					"type": "arrow"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1380,
			"versionNonce": 520428355,
			"isDeleted": false,
			"id": "JF5xnseiXzZfl90aPZAxO",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2681.8366516895435,
			"y": -496.73193062318705,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 579.8442848015584,
			"height": 0,
			"seed": 1178200611,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					289.9221424007792,
					0
				],
				[
					579.8442848015584,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1703,
			"versionNonce": 333409901,
			"isDeleted": false,
			"id": "T3NV1SPG_pQdjDlARx1v6",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2687.692619091554,
			"y": -545.5562738647745,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 568,
			"height": 44,
			"seed": 1112243597,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "ytFH8Su9",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1671,
			"versionNonce": 786873059,
			"isDeleted": false,
			"id": "ytFH8Su9",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2855.692619091554,
			"y": -540.5562738647745,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 232,
			"height": 34,
			"seed": 2019470787,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FFT<T: FftNum>",
			"rawText": "FFT<T: FftNum>",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "T3NV1SPG_pQdjDlARx1v6",
			"originalText": "FFT<T: FftNum>"
		},
		{
			"type": "rectangle",
			"version": 1965,
			"versionNonce": 606148813,
			"isDeleted": false,
			"id": "oVnOYphuv8XTB0qOP-cSz",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2686.282894554882,
			"y": -495.00679920247103,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 577,
			"height": 86,
			"seed": 1149125613,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "I8JCKZgD",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1879,
			"versionNonce": 2092131971,
			"isDeleted": false,
			"id": "I8JCKZgD",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2691.282894554882,
			"y": -490.00679920247103,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 564,
			"height": 76,
			"seed": 2087320931,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+new(planner: &mut RealFftPlanner<T>, length: usize) -> Self\n+get_length(&self) -> usize\n+create_real_buffer(&self) -> Vec<T>\n+create_complex_buffer(&self) -> Vec<Complex<T>>",
			"rawText": "+new(planner: &mut RealFftPlanner<T>, length: usize) -> Self\n+get_length(&self) -> usize\n+create_real_buffer(&self) -> Vec<T>\n+create_complex_buffer(&self) -> Vec<Complex<T>>",
			"baseline": 72,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "oVnOYphuv8XTB0qOP-cSz",
			"originalText": "+new(planner: &mut RealFftPlanner<T>, length: usize) -> Self\n+get_length(&self) -> usize\n+create_real_buffer(&self) -> Vec<T>\n+create_complex_buffer(&self) -> Vec<Complex<T>>"
		},
		{
			"type": "rectangle",
			"version": 2658,
			"versionNonce": 1736341293,
			"isDeleted": false,
			"id": "SdgNdKX4jszM9qVBNbCuz",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2686.1403385203334,
			"y": -567.4659866865072,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 570,
			"height": 29,
			"seed": 1430863437,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "ybd6PJ1O",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2686,
			"versionNonce": 791256611,
			"isDeleted": false,
			"id": "ybd6PJ1O",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2928.1403385203334,
			"y": -562.4659866865072,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 1839880451,
			"groupIds": [
				"Uzst2x2Vfdtr4u0RW1A5m"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "SdgNdKX4jszM9qVBNbCuz",
			"originalText": "<<trait>>"
		},
		{
			"type": "rectangle",
			"version": 1209,
			"versionNonce": 1751189901,
			"isDeleted": false,
			"id": "vwFhOHg9vFjuwN45q5bjL",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2687.7203383175383,
			"y": -357.2109195394771,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 584.7038827749059,
			"height": 158.66049192777754,
			"seed": 451234093,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 882,
			"versionNonce": 288874947,
			"isDeleted": false,
			"id": "OSJAAgqM3BajGmElCAknZ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2687.4525758289265,
			"y": -303.7824405056192,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 583.1316325362238,
			"height": 0,
			"seed": 1741745187,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					291.5658162681119,
					0
				],
				[
					583.1316325362238,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1416,
			"versionNonce": 671240173,
			"isDeleted": false,
			"id": "CmshnD28TrvmoK9BHWJVO",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2693.079554260651,
			"y": -353.9944142050151,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 574,
			"height": 44,
			"seed": 1797102477,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "wSkR0QkP",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1398,
			"versionNonce": 201109859,
			"isDeleted": false,
			"id": "wSkR0QkP",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2806.579554260651,
			"y": -348.9944142050151,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 347,
			"height": 34,
			"seed": 331706307,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "ForwardFFT<T: FftNum>",
			"rawText": "ForwardFFT<T: FftNum>",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "CmshnD28TrvmoK9BHWJVO",
			"originalText": "ForwardFFT<T: FftNum>"
		},
		{
			"type": "rectangle",
			"version": 1688,
			"versionNonce": 1147914829,
			"isDeleted": false,
			"id": "KE9-R7u5c5rbL66inNbjg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2694.301545601422,
			"y": -297.92251856454504,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 399,
			"height": 48,
			"seed": 157065709,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "QRuWwLus",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1047,
			"versionNonce": 1623624963,
			"isDeleted": false,
			"id": "tL61ZDc6Ldm3t_HGKWS6i",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2688.817104974538,
			"y": -240.91271017377267,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 582.1890544176981,
			"height": 0,
			"seed": 1101561699,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					291.09452720884906,
					0
				],
				[
					582.1890544176981,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1800,
			"versionNonce": 1584855213,
			"isDeleted": false,
			"id": "KO0hZhKW-f7urZEE2hfed",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2690.76696076772,
			"y": -235.2274599439529,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 576,
			"height": 29,
			"seed": 1789647949,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "El9o25aL",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1638,
			"versionNonce": 588510371,
			"isDeleted": false,
			"id": "QRuWwLus",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2699.301545601422,
			"y": -292.92251856454504,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 301,
			"height": 38,
			"seed": 1271438083,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-plan: Arc<dyn RealToComplex<T>>\n-length: usize",
			"rawText": "-plan: Arc<dyn RealToComplex<T>>\n-length: usize",
			"baseline": 34,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "KE9-R7u5c5rbL66inNbjg",
			"originalText": "-plan: Arc<dyn RealToComplex<T>>\n-length: usize"
		},
		{
			"type": "text",
			"version": 1614,
			"versionNonce": 1070422797,
			"isDeleted": false,
			"id": "El9o25aL",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2695.76696076772,
			"y": -230.2274599439529,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 555,
			"height": 19,
			"seed": 652528301,
			"groupIds": [
				"qSQYyWMo0SoWGY_DoZvBz"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+process(&self, input: &mut [T], output: &mut [Complex<T>])",
			"rawText": "+process(&self, input: &mut [T], output: &mut [Complex<T>])",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "KO0hZhKW-f7urZEE2hfed",
			"originalText": "+process(&self, input: &mut [T], output: &mut [Complex<T>])"
		},
		{
			"type": "rectangle",
			"version": 1272,
			"versionNonce": 1922263107,
			"isDeleted": false,
			"id": "jsIhDhWwJTRYx3Ak2o0mb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2688.9955212282484,
			"y": -156.4692692995659,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 584.7038827749059,
			"height": 159.76371756790726,
			"seed": 2087958541,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 944,
			"versionNonce": 1088975213,
			"isDeleted": false,
			"id": "e2f4a3KVaqyLmGMw8uu27",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2688.7277587396356,
			"y": -103.04079026570798,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 583.1316325362238,
			"height": 0,
			"seed": 2144164675,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					291.5658162681119,
					0
				],
				[
					583.1316325362238,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1478,
			"versionNonce": 1965254627,
			"isDeleted": false,
			"id": "NMTutBiLEzXY4q39l3bpV",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2694.354737171361,
			"y": -153.25276396510435,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 574,
			"height": 44,
			"seed": 851127917,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "GfpZB745",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1469,
			"versionNonce": 1863433165,
			"isDeleted": false,
			"id": "GfpZB745",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2807.854737171361,
			"y": -148.25276396510435,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 347,
			"height": 34,
			"seed": 1286011619,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "InverseFFT<T: FftNum>",
			"rawText": "InverseFFT<T: FftNum>",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "NMTutBiLEzXY4q39l3bpV",
			"originalText": "InverseFFT<T: FftNum>"
		},
		{
			"type": "rectangle",
			"version": 1757,
			"versionNonce": 282227587,
			"isDeleted": false,
			"id": "_JN-8a3VR-RWn6xPnuKaD",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2695.576728512131,
			"y": -97.1808683246336,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 399,
			"height": 67,
			"seed": 1904531661,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "UJAq9NYu",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1109,
			"versionNonce": 1919072813,
			"isDeleted": false,
			"id": "12sxNPzE2204Qoy20Wxtp",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2690.092287885248,
			"y": -40.171059933862125,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 582.1890544176981,
			"height": 0,
			"seed": 2020320899,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					291.09452720884906,
					0
				],
				[
					582.1890544176981,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1793,
			"versionNonce": 1709116195,
			"isDeleted": false,
			"id": "NM27c8b9Pg_7udKCm4HI7",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2692.0421436784295,
			"y": -34.48580970404237,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 576,
			"height": 29,
			"seed": 1382692653,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "y7MAjAEm",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1723,
			"versionNonce": 868543629,
			"isDeleted": false,
			"id": "UJAq9NYu",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2700.576728512131,
			"y": -92.1808683246336,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 301,
			"height": 38,
			"seed": 165273123,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-plan: Arc<dyn ComplexToReal<T>>\n-length: usize",
			"rawText": "-plan: Arc<dyn ComplexToReal<T>>\n-length: usize",
			"baseline": 34,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "_JN-8a3VR-RWn6xPnuKaD",
			"originalText": "-plan: Arc<dyn ComplexToReal<T>>\n-length: usize"
		},
		{
			"type": "text",
			"version": 1697,
			"versionNonce": 146255555,
			"isDeleted": false,
			"id": "y7MAjAEm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2697.0421436784295,
			"y": -29.485809704042367,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 555,
			"height": 19,
			"seed": 2144651661,
			"groupIds": [
				"bHOOsVRCXl67g1CBtMgk4"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+process(&self, input: &mut [Complex<T>], output: &mut [T])",
			"rawText": "+process(&self, input: &mut [Complex<T>], output: &mut [T])",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "NM27c8b9Pg_7udKCm4HI7",
			"originalText": "+process(&self, input: &mut [Complex<T>], output: &mut [T])"
		},
		{
			"type": "rectangle",
			"version": 1496,
			"versionNonce": 1123144429,
			"isDeleted": false,
			"id": "KbzSefVK9T6d6H2qgT1Q_",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 717.8473591421043,
			"y": -358.12928470882514,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 695,
			"height": 611,
			"seed": 895555587,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "2V00ZVIP",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1233,
			"versionNonce": 2080038499,
			"isDeleted": false,
			"id": "2V00ZVIP",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1047.8473591421043,
			"y": -353.12928470882514,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 35,
			"height": 34,
			"seed": 517905837,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "ui",
			"rawText": "ui",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "KbzSefVK9T6d6H2qgT1Q_",
			"originalText": "ui"
		},
		{
			"type": "rectangle",
			"version": 1014,
			"versionNonce": 2087906637,
			"isDeleted": false,
			"id": "RUNnG8r7FrL-HNbMIJvND",
			"fillStyle": "solid",
			"strokeWidth": 0.5,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 776.2141449558064,
			"y": -258.0349210384878,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 581,
			"height": 451,
			"seed": 1497162349,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "9ytaNbkv",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 763,
			"versionNonce": 1786190339,
			"isDeleted": false,
			"id": "9ytaNbkv",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1030.7141449558064,
			"y": -253.03492103848782,
			"strokeColor": "#000000",
			"backgroundColor": "#00000008",
			"width": 72,
			"height": 24,
			"seed": 256542435,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "editor",
			"rawText": "editor",
			"baseline": 19,
			"textAlign": "center",
			"verticalAlign": "top",
			"containerId": "RUNnG8r7FrL-HNbMIJvND",
			"originalText": "editor"
		},
		{
			"type": "rectangle",
			"version": 1448,
			"versionNonce": 1872870317,
			"isDeleted": false,
			"id": "DshKR8s9HddfBsmmWTrSb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 822.2423144683396,
			"y": -171.57220712031233,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 481.0006726026877,
			"height": 140.18097037745372,
			"seed": 2139359597,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1263,
			"versionNonce": 556458403,
			"isDeleted": false,
			"id": "modMMInsE-gjdTOyg-2U4",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 821.9745519797273,
			"y": -109.3179229654146,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 481.63487364426544,
			"height": 0,
			"seed": 833441763,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					481.63487364426544,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1888,
			"versionNonce": 78102029,
			"isDeleted": false,
			"id": "KOxnuy1c-A-SNhEaUEhk4",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 828.9512725286006,
			"y": -165.95154011435886,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 466,
			"height": 49,
			"seed": 108680141,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "SgjLIWYP",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1780,
			"versionNonce": 1229239619,
			"isDeleted": false,
			"id": "SgjLIWYP",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 937.9512725286006,
			"y": -158.45154011435886,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 248,
			"height": 34,
			"seed": 1944017795,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476629,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "FreqChainEditor",
			"rawText": "FreqChainEditor",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "KOxnuy1c-A-SNhEaUEhk4",
			"originalText": "FreqChainEditor"
		},
		{
			"type": "rectangle",
			"version": 1933,
			"versionNonce": 1652454509,
			"isDeleted": false,
			"id": "oUi9nPDfTuDoHuFqJjJsG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 828.8235217522226,
			"y": -103.4580010243402,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 467,
			"height": 67,
			"seed": 1090015789,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "2smsFtVk",
					"type": "text"
				}
			],
			"updated": 1734013476629,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1999,
			"versionNonce": 1646466275,
			"isDeleted": false,
			"id": "2smsFtVk",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 833.8235217522226,
			"y": -98.4580010243402,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 273,
			"height": 57,
			"seed": 1806341827,
			"groupIds": [
				"bJDHbWHyktpkY05BZBG7S"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "-params: Arc<FreqChainParams>\n-context: Arc<dyn GuiContext>\n-sample_rate: Arc<AtomicF32>",
			"rawText": "-params: Arc<FreqChainParams>\n-context: Arc<dyn GuiContext>\n-sample_rate: Arc<AtomicF32>",
			"baseline": 53,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "oUi9nPDfTuDoHuFqJjJsG",
			"originalText": "-params: Arc<FreqChainParams>\n-context: Arc<dyn GuiContext>\n-sample_rate: Arc<AtomicF32>"
		},
		{
			"type": "rectangle",
			"version": 1356,
			"versionNonce": 351231693,
			"isDeleted": false,
			"id": "D_yzFXOaTCLGWOh8Hq1ob",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 886.9415298949441,
			"y": 20.7630174965135,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 335.7934922157535,
			"height": 140.59160180070364,
			"seed": 1323489795,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true
		},
		{
			"type": "line",
			"version": 1146,
			"versionNonce": 2069135491,
			"isDeleted": false,
			"id": "v5WbxV9PhOfbkUbJIn3BW",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 887.5564896488247,
			"y": 125.99360599716499,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 334.7931205020727,
			"height": 0,
			"seed": 684850093,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "round",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					167.39656025103636,
					0
				],
				[
					334.7931205020727,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1504,
			"versionNonce": 1922301229,
			"isDeleted": false,
			"id": "r1xqS6VPN9g-Zx1WZMSLH",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 892.4162612970629,
			"y": 73.3753074136331,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 323,
			"height": 44,
			"seed": 35561891,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "mVpRoBZS",
					"type": "text"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1475,
			"versionNonce": 781678627,
			"isDeleted": false,
			"id": "mVpRoBZS",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 995.4162612970629,
			"y": 78.3753074136331,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 117,
			"height": 34,
			"seed": 768605709,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Message",
			"rawText": "Message",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "r1xqS6VPN9g-Zx1WZMSLH",
			"originalText": "Message"
		},
		{
			"type": "rectangle",
			"version": 1863,
			"versionNonce": 1121308557,
			"isDeleted": false,
			"id": "zlRpjiSrQHASaIq6wKWe6",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 890.1364139781911,
			"y": 126.50859865229938,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 326,
			"height": 29,
			"seed": 1438676291,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "VDYEU7YO",
					"type": "text"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 1630,
			"versionNonce": 1507490755,
			"isDeleted": false,
			"id": "VDYEU7YO",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 895.1364139781911,
			"y": 131.50859865229938,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 246,
			"height": 19,
			"seed": 1017033837,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "+ParamUpdate(ParamMessage)",
			"rawText": "+ParamUpdate(ParamMessage)",
			"baseline": 15,
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": "zlRpjiSrQHASaIq6wKWe6",
			"originalText": "+ParamUpdate(ParamMessage)"
		},
		{
			"type": "rectangle",
			"version": 2404,
			"versionNonce": 602822125,
			"isDeleted": false,
			"id": "XLs7rxTHgjjqwaUT2r403",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 893.0533380799332,
			"y": 25.430509925894057,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 325,
			"height": 29,
			"seed": 1764152547,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "BL3ZbE7B",
					"type": "text"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2398,
			"versionNonce": 423365475,
			"isDeleted": false,
			"id": "BL3ZbE7B",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1017.0533380799332,
			"y": 30.430509925894057,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 77,
			"height": 19,
			"seed": 1091018445,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<enum>>",
			"rawText": "<<enum>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "XLs7rxTHgjjqwaUT2r403",
			"originalText": "<<enum>>"
		},
		{
			"type": "rectangle",
			"version": 2468,
			"versionNonce": 820449357,
			"isDeleted": false,
			"id": "mvmBD3LN39BvoHLBnb-4H",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 892.1100328180867,
			"y": 51.862887676567084,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 324,
			"height": 29,
			"seed": 1809858691,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "UmD7E25g",
					"type": "text"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": true
		},
		{
			"type": "text",
			"version": 2510,
			"versionNonce": 17475331,
			"isDeleted": false,
			"id": "UmD7E25g",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 917.1100328180867,
			"y": 56.862887676567084,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 274,
			"height": 19,
			"seed": 1906588973,
			"groupIds": [
				"DMN_TC0yOUDF4qQhSRlJX"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": true,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "#[derive(Debug, Clone, Copy)]",
			"rawText": "#[derive(Debug, Clone, Copy)]",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "mvmBD3LN39BvoHLBnb-4H",
			"originalText": "#[derive(Debug, Clone, Copy)]"
		},
		{
			"type": "rectangle",
			"version": 1619,
			"versionNonce": 1467686371,
			"isDeleted": false,
			"id": "4LK6EoronKHDVSfJDMqhs",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 268.13433194268157,
			"y": -710.2792907121858,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 166.13384014714424,
			"height": 73.50945273923699,
			"seed": 230156995,
			"groupIds": [
				"RcRmOIyjT8ydeBgM-fBvS"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "VZK2GXyEEFvnXfj2iLY1P",
					"type": "arrow"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1832,
			"versionNonce": 2059626883,
			"isDeleted": false,
			"id": "tXZEkBjk9tf55GYw3yd4p",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 275.7984206988914,
			"y": -683.7020854610724,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 155,
			"height": 44,
			"seed": 975394541,
			"groupIds": [
				"RcRmOIyjT8ydeBgM-fBvS"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "vdKWz8pN",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1799,
			"versionNonce": 346301485,
			"isDeleted": false,
			"id": "vdKWz8pN",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 303.2984206988914,
			"y": -678.7020854610724,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 100,
			"height": 34,
			"seed": 1838389859,
			"groupIds": [
				"RcRmOIyjT8ydeBgM-fBvS"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Plugin",
			"rawText": "Plugin",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "tXZEkBjk9tf55GYw3yd4p",
			"originalText": "Plugin"
		},
		{
			"type": "rectangle",
			"version": 2712,
			"versionNonce": 306266403,
			"isDeleted": false,
			"id": "0ZjlAgavKh4bPBDWLe8M7",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 274.2461401276706,
			"y": -705.6117982828051,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 154,
			"height": 29,
			"seed": 450097485,
			"groupIds": [
				"RcRmOIyjT8ydeBgM-fBvS"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "FJ3dc3ia",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 2734,
			"versionNonce": 508106381,
			"isDeleted": false,
			"id": "FJ3dc3ia",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 308.2461401276706,
			"y": -700.6117982828051,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 563631619,
			"groupIds": [
				"RcRmOIyjT8ydeBgM-fBvS"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "0ZjlAgavKh4bPBDWLe8M7",
			"originalText": "<<trait>>"
		},
		{
			"type": "rectangle",
			"version": 1567,
			"versionNonce": 392373443,
			"isDeleted": false,
			"id": "cqPNbL7gZox_oDgMr2wCn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 238.54311398657342,
			"y": -417.52518531705493,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 229.6488978624767,
			"height": 73.50945273923699,
			"seed": 1710432355,
			"groupIds": [
				"ailCP8sWPGkxImQ2BkVIF"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1742,
			"versionNonce": 1600175341,
			"isDeleted": false,
			"id": "H5G0bkpIJG0kct6LzzhSB",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 246.20720274278324,
			"y": -390.9479800659416,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 215,
			"height": 44,
			"seed": 2123877197,
			"groupIds": [
				"ailCP8sWPGkxImQ2BkVIF"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "FaTSVJwb",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1690,
			"versionNonce": 1372034147,
			"isDeleted": false,
			"id": "FaTSVJwb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 270.70720274278324,
			"y": -385.9479800659416,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 166,
			"height": 34,
			"seed": 806820867,
			"groupIds": [
				"ailCP8sWPGkxImQ2BkVIF"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Vst3Plugin",
			"rawText": "Vst3Plugin",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "H5G0bkpIJG0kct6LzzhSB",
			"originalText": "Vst3Plugin"
		},
		{
			"type": "rectangle",
			"version": 2619,
			"versionNonce": 1064605517,
			"isDeleted": false,
			"id": "MgFC6VeEEficSH7J2IqWT",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 244.65492217156248,
			"y": -412.85769288767426,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 219,
			"height": 29,
			"seed": 1434114477,
			"groupIds": [
				"ailCP8sWPGkxImQ2BkVIF"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "d1Ihn8cW",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 2613,
			"versionNonce": 289214467,
			"isDeleted": false,
			"id": "d1Ihn8cW",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 311.1549221715625,
			"y": -407.85769288767426,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 2004525987,
			"groupIds": [
				"ailCP8sWPGkxImQ2BkVIF"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "MgFC6VeEEficSH7J2IqWT",
			"originalText": "<<trait>>"
		},
		{
			"type": "rectangle",
			"version": 1587,
			"versionNonce": 2027824557,
			"isDeleted": false,
			"id": "Uon7nBEec5k4aWT9PDcJ9",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 238.51396964527135,
			"y": -248.42474886582193,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 229.6488978624767,
			"height": 73.50945273923699,
			"seed": 1628807715,
			"groupIds": [
				"vTJGTt9rRIu1UYtaEQxU-"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1760,
			"versionNonce": 667031459,
			"isDeleted": false,
			"id": "FI67gZz6GAgKtc5gIh2mN",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 246.17805840148117,
			"y": -221.84754361470868,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 215,
			"height": 44,
			"seed": 505890189,
			"groupIds": [
				"vTJGTt9rRIu1UYtaEQxU-"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "voD3ySNH",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1713,
			"versionNonce": 1053222925,
			"isDeleted": false,
			"id": "voD3ySNH",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 270.67805840148117,
			"y": -216.84754361470868,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 166,
			"height": 34,
			"seed": 788751811,
			"groupIds": [
				"vTJGTt9rRIu1UYtaEQxU-"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "IcedEditor",
			"rawText": "IcedEditor",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "FI67gZz6GAgKtc5gIh2mN",
			"originalText": "IcedEditor"
		},
		{
			"type": "rectangle",
			"version": 2639,
			"versionNonce": 311870595,
			"isDeleted": false,
			"id": "R57qlrmHarLPBDb8bk0EY",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 244.6257778302604,
			"y": -243.75725643644125,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 219,
			"height": 29,
			"seed": 1101073389,
			"groupIds": [
				"vTJGTt9rRIu1UYtaEQxU-"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "RFkGQ9nf",
					"type": "text"
				}
			],
			"updated": 1734013521191,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 2626,
			"versionNonce": 1222953699,
			"isDeleted": false,
			"id": "RFkGQ9nf",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 311.1257778302604,
			"y": -238.75725643644125,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 1840756067,
			"groupIds": [
				"vTJGTt9rRIu1UYtaEQxU-"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "R57qlrmHarLPBDb8bk0EY",
			"originalText": "<<trait>>"
		},
		{
			"type": "arrow",
			"version": 769,
			"versionNonce": 745993357,
			"isDeleted": false,
			"id": "4pY8K_ZjojW6-ozxZD5iv",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 821.3350224133516,
			"y": -104.60132170885572,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 334.6566305004102,
			"height": 107.03739093197026,
			"seed": 583806275,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013526836,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-190.78255988951264,
					0
				],
				[
					-190.78255988951264,
					-107.03739093197026
				],
				[
					-334.6566305004102,
					-107.03739093197026
				]
			]
		},
		{
			"type": "arrow",
			"version": 1244,
			"versionNonce": 193684003,
			"isDeleted": false,
			"id": "38JRRy_gDquzUOviyqGyY",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1658.6555715859763,
			"y": -529.0177445094978,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 1176.5468138026276,
			"height": 0,
			"seed": 2027798691,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013514225,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "XgHztKyg8C9bBHqREncQT",
				"focus": -0.07433293589517583,
				"gap": 7.414255655486613
			},
			"endBinding": {
				"elementId": "-EfbZKtkW9ERdkquw4z_A",
				"focus": 0.11853324848372238,
				"gap": 18.164088799818614
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-1176.5468138026276,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 179,
			"versionNonce": 333560739,
			"isDeleted": false,
			"id": "GIqzQrHCD5pducA_UZqIc",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2973.3683029078975,
			"y": -356.80589132098095,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 30.807931569642392,
			"seed": 742577891,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-30.807931569642392
				]
			]
		},
		{
			"type": "arrow",
			"version": 350,
			"versionNonce": 1625523213,
			"isDeleted": false,
			"id": "jfKj0FtGcfJ0CnHmOecmd",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3275.678740551595,
			"y": -78.52932896658717,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 42.80421641903104,
			"height": 414.4090404413776,
			"seed": 1293012589,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					42.80421641903104,
					0
				],
				[
					42.80421641903104,
					-414.4090404413776
				],
				[
					6.215559652914642,
					-414.4090404413776
				]
			]
		},
		{
			"type": "rectangle",
			"version": 303,
			"versionNonce": 873484099,
			"isDeleted": false,
			"id": "TPMZ3KtLlLXibX0_8kGVE",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 659.6458895141412,
			"y": 1184.0118700730131,
			"strokeColor": "#000000",
			"backgroundColor": "#000",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 189133667,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false
		},
		{
			"type": "line",
			"version": 316,
			"versionNonce": 256997997,
			"isDeleted": false,
			"id": "Y9IVgg0uU-erXyIJdesSo",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3262.9220078588337,
			"y": -492.7001399424914,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 1321678883,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 365,
			"versionNonce": 1684762339,
			"isDeleted": false,
			"id": "y7oPE6OWqzKqaXA3Haanh",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 1.5707963267948957,
			"x": 2964.2100803766525,
			"y": -396.7370891270261,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 1663696803,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 439,
			"versionNonce": 1020004557,
			"isDeleted": false,
			"id": "7_uLshbR-KVFXKCKMK--Z",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 435.1781066953955,
			"y": -673.0093797145648,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 1562797805,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 484,
			"versionNonce": 1282373251,
			"isDeleted": false,
			"id": "0OjPfHorz_W9wRwfIH-GP",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 463.8038514770259,
			"y": -528.2965345175046,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 133677325,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 504,
			"versionNonce": 223045421,
			"isDeleted": false,
			"id": "-OQG7swe8ivxOxWvddIua",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 468.7902544039663,
			"y": -378.39280128176154,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 326951747,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 479,
			"versionNonce": 1289989667,
			"isDeleted": false,
			"id": "z_sk8cFZMyhOYKzI3styx",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 469.47368583625087,
			"y": -212.05821936785125,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 1176717987,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 255,
			"versionNonce": 939431309,
			"isDeleted": false,
			"id": "PHY7_-lqbdj6bnCbElnM-",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 1882.9849258601387,
			"y": -343.1898297984454,
			"strokeColor": "#000000",
			"backgroundColor": "#000",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 231636803,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "NlPidL6vIGpAMOqDsvxDt",
					"type": "arrow"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 53,
			"versionNonce": 1024005571,
			"isDeleted": false,
			"id": "NlPidL6vIGpAMOqDsvxDt",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1889.6859560792652,
			"y": -325.8003979978438,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 51.927011928301624,
			"seed": 2071900835,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "PHY7_-lqbdj6bnCbElnM-",
				"focus": 0.00930449211087422,
				"gap": 1
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					51.927011928301624
				]
			]
		},
		{
			"type": "arrow",
			"version": 239,
			"versionNonce": 2005158669,
			"isDeleted": false,
			"id": "VZK2GXyEEFvnXfj2iLY1P",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 660.6808362759673,
			"y": -528.7931449690711,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 208.43691457887297,
			"height": 143.90306558114355,
			"seed": 1929038947,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508853,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "4LK6EoronKHDVSfJDMqhs",
				"focus": 0.02253734074966219,
				"gap": 17.975749607268483
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-143.90306558114355
				],
				[
					-208.43691457887297,
					-143.90306558114355
				]
			]
		},
		{
			"type": "arrow",
			"version": 222,
			"versionNonce": 221245891,
			"isDeleted": false,
			"id": "_6KZ-x85pMzjYHqBLWIVB",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 660.6808362759673,
			"y": -528.0513765897869,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 173.57380075251342,
			"height": 149.0954442361333,
			"seed": 1560010147,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					149.0954442361333
				],
				[
					-173.57380075251342,
					149.0954442361333
				]
			]
		},
		{
			"type": "arrow",
			"version": 149,
			"versionNonce": 454448717,
			"isDeleted": false,
			"id": "ANBUF4izyvGpoh_thhi9u",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1633.6561370194263,
			"y": 893.0814850559982,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 190.2766905701942,
			"height": 108.68638926244171,
			"seed": 402709667,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-190.2766905701942,
					0
				],
				[
					-190.2766905701942,
					-108.68638926244171
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1656,
			"versionNonce": 324222957,
			"isDeleted": false,
			"id": "qTVcUohadhHD1MQpb8_xm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 237.35556683495548,
			"y": -100.29037613685836,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 229.6488978624767,
			"height": 73.50945273923699,
			"seed": 1362728717,
			"groupIds": [
				"gfi3NehzBFt3pGo4l7Zw8"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1824,
			"versionNonce": 1853014371,
			"isDeleted": false,
			"id": "u9QHIT8edwhnfiyvQleA6",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 245.0196555911653,
			"y": -73.71317088574511,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 215,
			"height": 44,
			"seed": 706276419,
			"groupIds": [
				"gfi3NehzBFt3pGo4l7Zw8"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "B4EChb8q",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1782,
			"versionNonce": 2015279693,
			"isDeleted": false,
			"id": "B4EChb8q",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 302.5196555911653,
			"y": -68.71317088574511,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 100,
			"height": 34,
			"seed": 1366109549,
			"groupIds": [
				"gfi3NehzBFt3pGo4l7Zw8"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 28,
			"fontFamily": 3,
			"text": "Params",
			"rawText": "Params",
			"baseline": 27,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "u9QHIT8edwhnfiyvQleA6",
			"originalText": "Params"
		},
		{
			"type": "rectangle",
			"version": 2698,
			"versionNonce": 704290051,
			"isDeleted": false,
			"id": "pIQXbHVGPOFkznBOYP_Bg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 243.46737501994454,
			"y": -95.62288370747768,
			"strokeColor": "transparent",
			"backgroundColor": "transparent",
			"width": 219,
			"height": 29,
			"seed": 693660643,
			"groupIds": [
				"gfi3NehzBFt3pGo4l7Zw8"
			],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "INxBGIo7",
					"type": "text"
				}
			],
			"updated": 1734013508756,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 2689,
			"versionNonce": 2118903981,
			"isDeleted": false,
			"id": "INxBGIo7",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 309.96737501994454,
			"y": -90.62288370747768,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 86,
			"height": 19,
			"seed": 127858637,
			"groupIds": [
				"gfi3NehzBFt3pGo4l7Zw8"
			],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 3,
			"text": "<<trait>>",
			"rawText": "<<trait>>",
			"baseline": 15,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "pIQXbHVGPOFkznBOYP_Bg",
			"originalText": "<<trait>>"
		},
		{
			"type": "arrow",
			"version": 443,
			"versionNonce": 1843010435,
			"isDeleted": false,
			"id": "AdyFVHtLT9hLL256-w1F2",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1679.188981651823,
			"y": -143.25972293651336,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 1331.7690782157022,
			"height": 473.7334193780176,
			"seed": 382523523,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013503215,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-151.24756413342698,
					0
				],
				[
					-151.24756413342698,
					473.7334193780176
				],
				[
					-1331.7690782157022,
					473.7334193780176
				],
				[
					-1331.7690782157022,
					135.9941947432921
				]
			]
		},
		{
			"type": "line",
			"version": 534,
			"versionNonce": 1262879907,
			"isDeleted": false,
			"id": "2q1pR1fU-3arSKIX21qvc",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 1.5707963267948957,
			"x": 338.79843117134703,
			"y": -17.986902128828348,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 17.281118533948757,
			"height": 15.422814644530376,
			"seed": 246325197,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013508756,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					17.069942675860148,
					-7.6038270039205145
				],
				[
					17.281118533948757,
					7.818987640609862
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 116,
			"versionNonce": 1175865293,
			"isDeleted": false,
			"id": "p0TmxmJNmg1K1MgKgHdzM",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1896.7831841949737,
			"y": -20.054282621604074,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 62.76361512829445,
			"seed": 907484547,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "g3nQV5fV3GfJZtg1VCLD6",
				"focus": 0.040014251783988986,
				"gap": 1.249179287225445
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					62.76361512829445
				]
			]
		},
		{
			"type": "rectangle",
			"version": 302,
			"versionNonce": 53258115,
			"isDeleted": false,
			"id": "g3nQV5fV3GfJZtg1VCLD6",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 1890.3770613311447,
			"y": -37.83006169933655,
			"strokeColor": "#000000",
			"backgroundColor": "#000",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 714603981,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "NlPidL6vIGpAMOqDsvxDt",
					"type": "arrow"
				},
				{
					"id": "p0TmxmJNmg1K1MgKgHdzM",
					"type": "arrow"
				}
			],
			"updated": 1734013476630,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 567,
			"versionNonce": 284542509,
			"isDeleted": false,
			"id": "eJ7m0dO_Z_iyES9IdHqOQ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 488.3114815616783,
			"y": 1047.9745949286787,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 1727.6651353706864,
			"height": 1038.535457475907,
			"seed": 128814253,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-319.76139463759114,
					0
				],
				[
					-319.76139463759114,
					-674.0500965690703
				],
				[
					1078.32248672935,
					-676.5386771742653
				],
				[
					1078.32248672935,
					-1038.535457475907
				],
				[
					1407.9037407330952,
					-1038.535457475907
				]
			]
		},
		{
			"type": "arrow",
			"version": 540,
			"versionNonce": 1993522979,
			"isDeleted": false,
			"id": "pe6mWjtnpmlpLujHmcjKE",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1312.7204827380426,
			"y": 784.3354801845572,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 576.5073088361687,
			"height": 1085.5528300724454,
			"seed": 51070189,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					130.6216524737397,
					0
				],
				[
					131.20767961888805,
					-405.1637743488967
				],
				[
					130.2392316057269,
					-405.6600367509118
				],
				[
					129.49199565463505,
					-405.7711871095337
				],
				[
					128.75924736962907,
					-405.95502377520916
				],
				[
					128.04803951779172,
					-406.20977730376364
				],
				[
					127.365217537631,
					-406.532995669942
				],
				[
					126.71735365112727,
					-406.9215678683687
				],
				[
					126.11068360551667,
					-407.3717538572498
				],
				[
					125.55104665367162,
					-407.879220556606
				],
				[
					125.04382935077501,
					-408.4390835545513
				],
				[
					124.59391370825006,
					-409.04595412018887
				],
				[
					124.20563020396821,
					-409.6939910706234
				],
				[
					123.88271610101697,
					-410.37695699286337
				],
				[
					123.62827947621145,
					-411.08827827947266
				],
				[
					123.44476930457995,
					-411.8211084001239
				],
				[
					123.33395188776285,
					-412.56839380005874
				],
				[
					123.29689385320307,
					-413.32294179117696
				],
				[
					123.33395188776285,
					-414.0774897822952
				],
				[
					123.44476930457995,
					-414.82477518223004
				],
				[
					123.62827947621145,
					-415.55760530288126
				],
				[
					123.88271610101697,
					-416.26892658949055
				],
				[
					124.20563020396821,
					-416.95189251173053
				],
				[
					124.59391370825006,
					-417.59992946216516
				],
				[
					125.04382935077501,
					-418.2068000278026
				],
				[
					125.55104665367162,
					-418.7666630257479
				],
				[
					126.11068360551667,
					-419.2741297251041
				],
				[
					126.71735365112727,
					-419.7243157139852
				],
				[
					127.365217537631,
					-420.1128879124119
				],
				[
					128.04803951779172,
					-420.4361062785903
				],
				[
					128.75924736962907,
					-420.69085980714476
				],
				[
					129.49199565463505,
					-420.87469647282023
				],
				[
					130.2392316057269,
					-420.98584683144213
				],
				[
					130.99376301075358,
					-421.0232410509733
				],
				[
					130.99376301075358,
					-1085.5528300724454
				],
				[
					576.5073088361687,
					-1085.5528300724454
				]
			]
		},
		{
			"type": "arrow",
			"version": 186,
			"versionNonce": 605452429,
			"isDeleted": false,
			"id": "bxTlYHQwJlznuc5GW6E-e",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1767.0998345363473,
			"y": 1262.1079932544062,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 352.25071685023454,
			"height": 928.1040626575746,
			"seed": 1380744653,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-352.25071685023454,
					0
				],
				[
					-352.25071685023454,
					-928.1040626575746
				]
			]
		},
		{
			"type": "arrow",
			"version": 331,
			"versionNonce": 689234627,
			"isDeleted": false,
			"id": "nYGWnfgIQEwsf5YYsl_KW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 483.2293182922243,
			"y": 1423.9309605118233,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 136.33832917167456,
			"height": 1095.2189953947632,
			"seed": 1092460205,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-136.33832917167456,
					0
				],
				[
					-136.33832917167456,
					-1095.2189953947632
				]
			]
		},
		{
			"type": "arrow",
			"version": 51,
			"versionNonce": 1228005101,
			"isDeleted": false,
			"id": "bybUSMgBqwpBDBbsU8mDn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 487.31586870279466,
			"y": 1098.8374555855553,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 141.66624356516775,
			"height": 0,
			"seed": 1294726349,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-141.66624356516775,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 187,
			"versionNonce": 43094627,
			"isDeleted": false,
			"id": "DkhN1b0AnzOX3SUJvCsi3",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2790.7379629908564,
			"y": 1010.1337289583064,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 1262.8492077285136,
			"height": 679.9957272384304,
			"seed": 796321347,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-304.67341025618,
					0
				],
				[
					-304.67341025618,
					-677.7879489032405
				],
				[
					-1262.8492077285136,
					-679.9957272384304
				]
			]
		},
		{
			"type": "arrow",
			"version": 58,
			"versionNonce": 1456568653,
			"isDeleted": false,
			"id": "86L4OZOJU8_yLSi0C4cJH",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1908.65899996618,
			"y": 213.0291663425324,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 116.66666666666669,
			"seed": 1444083747,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					116.66666666666669
				]
			]
		},
		{
			"type": "arrow",
			"version": 82,
			"versionNonce": 1410813443,
			"isDeleted": false,
			"id": "tEA1y96MLJfluJzqftCWM",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 844.8812221884025,
			"y": 1094.736237049603,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 56.36363636363649,
			"height": 195.5820357175653,
			"seed": 1663359405,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476630,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "wDmJWhIUiulY_hQwY9nk5",
				"focus": -0.08626737415632214,
				"gap": 1.4191496081195396
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					56.36363636363649,
					0
				],
				[
					56.36363636363649,
					-195.5820357175653
				]
			]
		},
		{
			"type": "rectangle",
			"version": 261,
			"versionNonce": 374261677,
			"isDeleted": false,
			"id": "wDmJWhIUiulY_hQwY9nk5",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 893.6260469298118,
			"y": 881.5822189645812,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 1850506477,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tEA1y96MLJfluJzqftCWM",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 174,
			"versionNonce": 1352963491,
			"isDeleted": false,
			"id": "9KqVafud2A4m-BbUf22Dc",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1174.5337434242733,
			"y": 664.1738399704079,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 1797.4677002219883,
			"height": 426.7629549822831,
			"seed": 139953123,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					0,
					-157.32363170957046
				],
				[
					1797.4677002219883,
					-157.32363170957046
				],
				[
					1797.4677002219883,
					-426.7629549822831
				]
			]
		},
		{
			"type": "arrow",
			"version": 81,
			"versionNonce": 1775454733,
			"isDeleted": false,
			"id": "BZRwuczWffojdNUguHRRz",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1992.0263277837703,
			"y": 1147.8159499966887,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 41.890555523423245,
			"seed": 1610018637,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "ycI8rYQGEZEBCPKk6PHpQ",
				"focus": 7.065206970589239e-17,
				"gap": 1
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-41.890555523423245
				]
			]
		},
		{
			"type": "rectangle",
			"version": 291,
			"versionNonce": 1197992259,
			"isDeleted": false,
			"id": "ycI8rYQGEZEBCPKk6PHpQ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 1985.235946062385,
			"y": 1089.330840182568,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 501956781,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tEA1y96MLJfluJzqftCWM",
					"type": "arrow"
				},
				{
					"id": "BZRwuczWffojdNUguHRRz",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 421,
			"versionNonce": 844660845,
			"isDeleted": false,
			"id": "l5dy0oeFZmsSy5BLKUySI",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2366.3628023953047,
			"y": 887.3644196105871,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 321.1705118844186,
			"height": 957.9743547989204,
			"seed": 1534805069,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					88.49855597783107,
					0
				],
				[
					88.62316026001781,
					-371.9487821925667
				],
				[
					87.65471224685666,
					-372.44504459458176
				],
				[
					86.90747629576481,
					-372.55619495320366
				],
				[
					86.17472801075883,
					-372.74003161887913
				],
				[
					85.46352015892148,
					-372.9947851474336
				],
				[
					84.78069817876076,
					-373.318003513612
				],
				[
					84.13283429225703,
					-373.70657571203867
				],
				[
					83.52616424664643,
					-374.1567617009198
				],
				[
					82.96652729480138,
					-374.664228400276
				],
				[
					82.45930999190477,
					-375.2240913982213
				],
				[
					82.00939434937982,
					-375.83096196385884
				],
				[
					81.62111084509797,
					-376.47899891429336
				],
				[
					81.29819674214673,
					-377.16196483653334
				],
				[
					81.04376011734121,
					-377.87328612314263
				],
				[
					80.86024994570971,
					-378.60611624379385
				],
				[
					80.74943252889261,
					-379.3534016437287
				],
				[
					80.71237449433283,
					-380.10794963484693
				],
				[
					80.74943252889261,
					-380.86249762596515
				],
				[
					80.86024994570971,
					-381.6097830259
				],
				[
					81.04376011734121,
					-382.34261314655123
				],
				[
					81.29819674214673,
					-383.0539344331605
				],
				[
					81.62111084509797,
					-383.7369003554005
				],
				[
					82.00939434937982,
					-384.38493730583514
				],
				[
					82.45930999190477,
					-384.9918078714726
				],
				[
					82.96652729480138,
					-385.5516708694179
				],
				[
					83.52616424664643,
					-386.0591375687741
				],
				[
					84.13283429225703,
					-386.5093235576552
				],
				[
					84.78069817876076,
					-386.8978957560819
				],
				[
					85.46352015892148,
					-387.22111412226025
				],
				[
					86.17472801075883,
					-387.47586765081473
				],
				[
					86.90747629576481,
					-387.6597043164902
				],
				[
					87.65471224685666,
					-387.7708546751121
				],
				[
					88.40924365188334,
					-387.80824889464327
				],
				[
					88.40924365188334,
					-957.9743547989204
				],
				[
					321.1705118844186,
					-957.9743547989204
				]
			]
		},
		{
			"type": "arrow",
			"version": 109,
			"versionNonce": 499108067,
			"isDeleted": false,
			"id": "TrxLogRQKWg-LrG6Hy8NW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2454.9533354476853,
			"y": -70.60993518833345,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 230.72675190907376,
			"height": 203.85496152608926,
			"seed": 1123591587,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					0,
					-203.85496152608926
				],
				[
					230.72675190907376,
					-203.85496152608926
				]
			]
		},
		{
			"type": "arrow",
			"version": 68,
			"versionNonce": 1973938893,
			"isDeleted": false,
			"id": "odu38JP7L1NFLi6NNqcrN",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2454.4379100192805,
			"y": 781.6090790638015,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 123.14771150002753,
			"height": 0,
			"seed": 763125325,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					123.14771150002753,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 207,
			"versionNonce": 154842243,
			"isDeleted": false,
			"id": "2OnjcHDwjlvY0vP2LRvx4",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2789.320791761689,
			"y": 1049.3754251400808,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 546.4778482491379,
			"height": 200.79619598465,
			"seed": 2021529773,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-310.32139379445925,
					0
				],
				[
					-310.32139379445925,
					200.79619598465
				],
				[
					-546.4778482491379,
					200.79619598465
				]
			]
		},
		{
			"type": "rectangle",
			"version": 360,
			"versionNonce": 1146816813,
			"isDeleted": false,
			"id": "gKPXjEv9KFdkhTj5ODDxV",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 2226.671950099348,
			"y": 1243.2773372228505,
			"strokeColor": "#000000",
			"backgroundColor": "#000",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 872153677,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tEA1y96MLJfluJzqftCWM",
					"type": "arrow"
				},
				{
					"id": "BZRwuczWffojdNUguHRRz",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 103,
			"versionNonce": 160902179,
			"isDeleted": false,
			"id": "aP01eZ7gXYaKAqRR539Jv",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2913.378258396182,
			"y": 907.0487343458075,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 45.24427483176157,
			"seed": 580345795,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-45.24427483176157
				]
			]
		},
		{
			"type": "rectangle",
			"version": 329,
			"versionNonce": 1608211341,
			"isDeleted": false,
			"id": "C_kQE3sHSFDq0k1KHXjyz",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 2906.5878766747965,
			"y": 846.4694406086306,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 962796099,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "tEA1y96MLJfluJzqftCWM",
					"type": "arrow"
				},
				{
					"id": "BZRwuczWffojdNUguHRRz",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 130,
			"versionNonce": 583249859,
			"isDeleted": false,
			"id": "qrDSaLbgEOWRAsJ76sSZX",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 666.4068536405184,
			"y": 1291.6068354118277,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 91.90027498376458,
			"seed": 1162028803,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-91.90027498376458
				]
			]
		},
		{
			"type": "arrow",
			"version": 287,
			"versionNonce": 543318509,
			"isDeleted": false,
			"id": "0A58SHh3GFURe_YMjmItW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1131.88663996542,
			"y": 1271.990250536323,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 76.81609021903296,
			"seed": 655368557,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "EqaUNW2_EMjxxAdX5yXLs",
				"focus": 0.0016712164252724565,
				"gap": 1
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-76.81609021903296
				]
			]
		},
		{
			"type": "rectangle",
			"version": 403,
			"versionNonce": 1457438563,
			"isDeleted": false,
			"id": "EqaUNW2_EMjxxAdX5yXLs",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 1125.080209469269,
			"y": 1274.948576761057,
			"strokeColor": "#000000",
			"backgroundColor": "transparent",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 970335213,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "0A58SHh3GFURe_YMjmItW",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 188,
			"versionNonce": 452566093,
			"isDeleted": false,
			"id": "dx-3RUyIO_HZ6cEyXTOXI",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1313.305600747914,
			"y": 1441.4085283697405,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 2249.163150271295,
			"height": 1273.8623151979016,
			"seed": 695457923,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					2249.163150271295,
					0
				],
				[
					2249.163150271295,
					-1273.8623151979016
				],
				[
					2044.236398475724,
					-1273.8623151979016
				]
			]
		},
		{
			"type": "rectangle",
			"version": 335,
			"versionNonce": 1614207747,
			"isDeleted": false,
			"id": "Ffmw6X9H4_dBFaoe2nxl_",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0.7853981633974474,
			"x": 1043.8611573928665,
			"y": -28.279397966813764,
			"strokeColor": "#000000",
			"backgroundColor": "#000",
			"width": 13.580763442770921,
			"height": 13.580763442770921,
			"seed": 164790115,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [
				{
					"id": "hPAL4k7N4K8UzXbuRDPxc",
					"type": "arrow"
				}
			],
			"updated": 1734013476631,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 45,
			"versionNonce": 125471405,
			"isDeleted": false,
			"id": "hPAL4k7N4K8UzXbuRDPxc",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1050.3829020525786,
			"y": 20.419530035589688,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 32.3276480849955,
			"seed": 1041937667,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"boundElements": [],
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "Ffmw6X9H4_dBFaoe2nxl_",
				"focus": 0.027974139861555322,
				"gap": 1
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					-32.3276480849955
				]
			]
		},
		{
			"id": "c1O5EIWT",
			"type": "text",
			"x": 1824.16293008693,
			"y": -615.803225359327,
			"width": 18,
			"height": 34,
			"angle": 0,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"seed": 915537827,
			"version": 9,
			"versionNonce": 726303395,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"text": "",
			"rawText": "",
			"fontSize": 28,
			"fontFamily": 3,
			"textAlign": "left",
			"verticalAlign": "top",
			"baseline": 27,
			"containerId": null,
			"originalText": ""
		},
		{
			"id": "FB1wNxuu",
			"type": "text",
			"x": 1886.8603192216847,
			"y": -630.6526069965057,
			"width": 18,
			"height": 34,
			"angle": 0,
			"strokeColor": "#000",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"strokeSharpness": "sharp",
			"seed": 864592077,
			"version": 9,
			"versionNonce": 1720626445,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1734013476631,
			"link": null,
			"locked": false,
			"text": "",
			"rawText": "",
			"fontSize": 28,
			"fontFamily": 3,
			"textAlign": "left",
			"verticalAlign": "top",
			"baseline": 27,
			"containerId": null,
			"originalText": ""
		}
	],
	"appState": {
		"theme": "dark",
		"viewBackgroundColor": "#ffffff",
		"currentItemStrokeColor": "#000",
		"currentItemBackgroundColor": "transparent",
		"currentItemFillStyle": "solid",
		"currentItemStrokeWidth": 2,
		"currentItemStrokeStyle": "solid",
		"currentItemRoughness": 0,
		"currentItemOpacity": 100,
		"currentItemFontFamily": 3,
		"currentItemFontSize": 28,
		"currentItemTextAlign": "left",
		"currentItemStrokeSharpness": "sharp",
		"currentItemStartArrowhead": null,
		"currentItemEndArrowhead": null,
		"currentItemLinearStrokeSharpness": "sharp",
		"gridSize": null,
		"colorPalette": {}
	},
	"files": {}
}
```
%%